-- Add generated_content column to intello_study_sessions table
ALTER TABLE intello_study_sessions
ADD COLUMN generated_content JSONB DEFAULT NULL;

COMMENT ON COLUMN intello_study_sessions.generated_content IS 'Stores the AI-generated course content for this session to avoid regeneration.';
