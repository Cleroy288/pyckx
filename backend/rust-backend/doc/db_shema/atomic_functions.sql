-- ============================================================================
-- ATOMIC GAME SET CREATION FUNCTIONS
-- ============================================================================
-- These PostgreSQL functions provide atomic (transactional) creation of
-- game sets with their associated items, preventing orphan data.
--
-- Run these migrations in your Supabase SQL Editor.
-- ============================================================================

-- ============================================================================
-- QCM SET (Questions + Answers)
-- ============================================================================

CREATE OR REPLACE FUNCTION create_qcm_set_atomic(
    p_set jsonb,
    p_questions jsonb
) RETURNS jsonb
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
    v_set_id uuid;
    v_result jsonb;
BEGIN
    -- Insert set
    INSERT INTO qcm_sets (id, user_id, name, description, level, language, subjects)
    VALUES (
        (p_set->>'id')::uuid,
        (p_set->>'user_id')::uuid,
        p_set->>'name',
        p_set->>'description',
        (p_set->>'level')::level_type,
        p_set->>'language',
        ARRAY(SELECT jsonb_array_elements_text(p_set->'subjects'))
    )
    RETURNING id INTO v_set_id;
    
    -- Insert questions
    INSERT INTO qcm_questions (id, set_id, question, wrong_answers, right_answer, explanation)
    SELECT 
        (q->>'id')::uuid,
        v_set_id,
        q->>'question',
        ARRAY(SELECT jsonb_array_elements_text(q->'wrong_answers')),
        q->>'right_answer',
        COALESCE(q->>'explanation', '')
    FROM jsonb_array_elements(p_questions) AS q;
    
    -- Return created set with questions count
    SELECT jsonb_build_object(
        'id', v_set_id,
        'questions_count', jsonb_array_length(p_questions)
    ) INTO v_result;
    
    RETURN v_result;
END;
$$;

-- ============================================================================
-- FLASHCARD SET (Cards)
-- ============================================================================

CREATE OR REPLACE FUNCTION create_flashcard_set_atomic(
    p_set jsonb,
    p_cards jsonb
) RETURNS jsonb
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
    v_set_id uuid;
    v_result jsonb;
BEGIN
    INSERT INTO flashcard_sets (id, user_id, name, description, level, language, subjects)
    VALUES (
        (p_set->>'id')::uuid,
        (p_set->>'user_id')::uuid,
        p_set->>'name',
        p_set->>'description',
        (p_set->>'level')::level_type,
        p_set->>'language',
        ARRAY(SELECT jsonb_array_elements_text(p_set->'subjects'))
    )
    RETURNING id INTO v_set_id;
    
    INSERT INTO flashcards (id, set_id, front, back)
    SELECT 
        (c->>'id')::uuid,
        v_set_id,
        c->>'front',
        c->>'back'
    FROM jsonb_array_elements(p_cards) AS c;
    
    SELECT jsonb_build_object('id', v_set_id, 'cards_count', jsonb_array_length(p_cards)) INTO v_result;
    RETURN v_result;
END;
$$;

-- ============================================================================
-- OPEN QUESTION SET (Questions)
-- ============================================================================

CREATE OR REPLACE FUNCTION create_open_question_set_atomic(
    p_set jsonb,
    p_questions jsonb
) RETURNS jsonb
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
    v_set_id uuid;
    v_result jsonb;
BEGIN
    INSERT INTO open_question_sets (id, user_id, name, description, level, language, subjects)
    VALUES (
        (p_set->>'id')::uuid,
        (p_set->>'user_id')::uuid,
        p_set->>'name',
        p_set->>'description',
        (p_set->>'level')::level_type,
        p_set->>'language',
        ARRAY(SELECT jsonb_array_elements_text(p_set->'subjects'))
    )
    RETURNING id INTO v_set_id;
    
    INSERT INTO open_questions (id, set_id, question, user_answer, expected_answer, hint)
    SELECT 
        (q->>'id')::uuid,
        v_set_id,
        q->>'question',
        COALESCE(q->>'user_answer', ''),
        q->>'expected_answer',
        q->>'hint'
    FROM jsonb_array_elements(p_questions) AS q;
    
    SELECT jsonb_build_object('id', v_set_id, 'questions_count', jsonb_array_length(p_questions)) INTO v_result;
    RETURN v_result;
END;
$$;

-- ============================================================================
-- TRUE/FALSE SET (Statements)
-- ============================================================================

CREATE OR REPLACE FUNCTION create_true_false_set_atomic(
    p_set jsonb,
    p_statements jsonb
) RETURNS jsonb
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
    v_set_id uuid;
    v_result jsonb;
BEGIN
    INSERT INTO intello_true_false_sets (id, user_id, name, description, level, language, subjects)
    VALUES (
        (p_set->>'id')::uuid,
        (p_set->>'user_id')::uuid,
        p_set->>'name',
        p_set->>'description',
        (p_set->>'level')::level_type,
        p_set->>'language',
        ARRAY(SELECT jsonb_array_elements_text(p_set->'subjects'))
    )
    RETURNING id INTO v_set_id;
    
    INSERT INTO intello_true_false_statements (id, set_id, statement, answer, explanation)
    SELECT 
        (s->>'id')::uuid,
        v_set_id,
        s->>'statement',
        (s->>'answer')::boolean,
        COALESCE(s->>'explanation', '')
    FROM jsonb_array_elements(p_statements) AS s;
    
    SELECT jsonb_build_object('id', v_set_id, 'statements_count', jsonb_array_length(p_statements)) INTO v_result;
    RETURN v_result;
END;
$$;

-- ============================================================================
-- FILL BLANK SET (Questions + Options) - 3 tables
-- ============================================================================

CREATE OR REPLACE FUNCTION create_fill_blank_set_atomic(
    p_set jsonb,
    p_questions jsonb
) RETURNS jsonb
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
    v_set_id uuid;
    v_question record;
    v_result jsonb;
BEGIN
    INSERT INTO intello_fill_blank_sets (id, user_id, name, description, level, language, subjects)
    VALUES (
        (p_set->>'id')::uuid,
        (p_set->>'user_id')::uuid,
        p_set->>'name',
        p_set->>'description',
        (p_set->>'level')::level_type,
        p_set->>'language',
        ARRAY(SELECT jsonb_array_elements_text(p_set->'subjects'))
    )
    RETURNING id INTO v_set_id;
    
    -- Process each question and its options
    FOR v_question IN SELECT * FROM jsonb_array_elements(p_questions)
    LOOP
        -- Insert question
        INSERT INTO intello_fill_blanks (id, set_id, phrase, explanation)
        VALUES (
            (v_question.value->>'id')::uuid,
            v_set_id,
            v_question.value->>'phrase',
            COALESCE(v_question.value->>'explanation', '')
        );
        
        -- Insert options for this question
        INSERT INTO intello_fill_blank_options (id, question_id, text, is_correct)
        SELECT 
            (o->>'id')::uuid,
            (v_question.value->>'id')::uuid,
            o->>'text',
            (o->>'is_correct')::boolean
        FROM jsonb_array_elements(v_question.value->'options') AS o;
    END LOOP;
    
    SELECT jsonb_build_object('id', v_set_id, 'questions_count', jsonb_array_length(p_questions)) INTO v_result;
    RETURN v_result;
END;
$$;

-- ============================================================================
-- KEYWORDS SET (Questions + Keywords) - 3 tables
-- ============================================================================

CREATE OR REPLACE FUNCTION create_keywords_set_atomic(
    p_set jsonb,
    p_questions jsonb
) RETURNS jsonb
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
    v_set_id uuid;
    v_question record;
    v_result jsonb;
BEGIN
    INSERT INTO intello_keyword_sets (id, user_id, name, description, level, language, subjects)
    VALUES (
        (p_set->>'id')::uuid,
        (p_set->>'user_id')::uuid,
        p_set->>'name',
        p_set->>'description',
        (p_set->>'level')::level_type,
        p_set->>'language',
        ARRAY(SELECT jsonb_array_elements_text(p_set->'subjects'))
    )
    RETURNING id INTO v_set_id;
    
    FOR v_question IN SELECT * FROM jsonb_array_elements(p_questions)
    LOOP
        INSERT INTO intello_keyword_questions (id, set_id, statement, explanation)
        VALUES (
            (v_question.value->>'id')::uuid,
            v_set_id,
            v_question.value->>'statement',
            COALESCE(v_question.value->>'explanation', '')
        );
        
        INSERT INTO intello_keywords (id, question_id, word, is_correct)
        SELECT 
            (k->>'id')::uuid,
            (v_question.value->>'id')::uuid,
            k->>'word',
            (k->>'is_correct')::boolean
        FROM jsonb_array_elements(v_question.value->'keywords') AS k;
    END LOOP;
    
    SELECT jsonb_build_object('id', v_set_id, 'questions_count', jsonb_array_length(p_questions)) INTO v_result;
    RETURN v_result;
END;
$$;

-- ============================================================================
-- ORDER PHRASE SET (Questions + Words) - 3 tables
-- ============================================================================

CREATE OR REPLACE FUNCTION create_order_phrase_set_atomic(
    p_set jsonb,
    p_questions jsonb
) RETURNS jsonb
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
    v_set_id uuid;
    v_question record;
    v_result jsonb;
BEGIN
    INSERT INTO intello_order_phrase_sets (id, user_id, name, description, level, language, subjects)
    VALUES (
        (p_set->>'id')::uuid,
        (p_set->>'user_id')::uuid,
        p_set->>'name',
        p_set->>'description',
        (p_set->>'level')::level_type,
        p_set->>'language',
        ARRAY(SELECT jsonb_array_elements_text(p_set->'subjects'))
    )
    RETURNING id INTO v_set_id;
    
    FOR v_question IN SELECT * FROM jsonb_array_elements(p_questions)
    LOOP
        INSERT INTO intello_order_phrase_questions (id, set_id, original_phrase, hint)
        VALUES (
            (v_question.value->>'id')::uuid,
            v_set_id,
            v_question.value->>'original_phrase',
            COALESCE(v_question.value->>'hint', '')
        );
        
        INSERT INTO intello_order_phrase_words (id, question_id, word, position)
        SELECT 
            (w->>'id')::uuid,
            (v_question.value->>'id')::uuid,
            w->>'word',
            (w->>'position')::int
        FROM jsonb_array_elements(v_question.value->'words') AS w;
    END LOOP;
    
    SELECT jsonb_build_object('id', v_set_id, 'questions_count', jsonb_array_length(p_questions)) INTO v_result;
    RETURN v_result;
END;
$$;

-- ============================================================================
-- GRANT PERMISSIONS
-- ============================================================================
-- Allow authenticated users to call these functions

GRANT EXECUTE ON FUNCTION create_qcm_set_atomic(jsonb, jsonb) TO authenticated;
GRANT EXECUTE ON FUNCTION create_flashcard_set_atomic(jsonb, jsonb) TO authenticated;
GRANT EXECUTE ON FUNCTION create_open_question_set_atomic(jsonb, jsonb) TO authenticated;
GRANT EXECUTE ON FUNCTION create_true_false_set_atomic(jsonb, jsonb) TO authenticated;
GRANT EXECUTE ON FUNCTION create_fill_blank_set_atomic(jsonb, jsonb) TO authenticated;
GRANT EXECUTE ON FUNCTION create_keywords_set_atomic(jsonb, jsonb) TO authenticated;
GRANT EXECUTE ON FUNCTION create_order_phrase_set_atomic(jsonb, jsonb) TO authenticated;

-- ============================================================================
-- USAGE EXAMPLE (from Rust)
-- ============================================================================
-- To call from Supabase REST API:
-- POST /rest/v1/rpc/create_qcm_set_atomic
-- Body: { "p_set": {...}, "p_questions": [...] }
