/*
 *
 * le mode learn c'est quand un user veut apprendre un truc
 * on dois donc continuer à lui générer des cours tant que il n'a pas 90% de réussite.
 *
 * on doit donc continuer à lui générer des qcm , etc tant que il n'a pas 90% de réussite
 *
 * quand un user fait un cours à la fin on lui propose un mode learn,
 * ça veut dire que on lui laisse le cours affiché et on lui génère une section extra sous la synthèse
 * et donc on lui génère un nouveau text (synthèse à apprendre)
 * et on lui génère des jeux pour apprendre ce text
 * puis une fois terminé le joueur nous envoi ses résultats, si il a plus de 90% on arrête le mode learn
 * sinon on re généère des jeux jusqu'a ce que il atteigne les 90%
 *
 * le jeux pour le mode learn devraient avoir plus d'info que les jeux normaux, le but est que le user apprend
 * vraiment avec ces jeux
 *
 *
 * process :
 * - 1 quand un user clique sur "learn mode" en dessou d'un cours, on récupère le résultat de ses jeux pour ce cours
 * on récupère aussi le sujet de son cours etc ...
 * - 2 on génère un text à apprendre (synthèse)
 * avec des jeux dedans , qcm, frai faux et aussi openquestion
 * les jeux doivent avoir plus de text et explications que les jeux normaux
 * - 3 à la fin on réucpère les résultats du user, on analayse ses erreurs, et si il n'a pas 90%
 * on re généère une synthèse basé sur ses erreurs et on re génère des jeux
 * - 4 une fois que le user à réussi on lui génère une nouvelle synthèse (de fin)
 *
 *
 */

// used to track user game res
struct UserGameResult {
    id: u32,                 // id res
    user_id: u32,            // user id, UUID
    game_id: u32,            // game id
    game_type: String,       // type of game (qcm, fill in the blank, etc)
    result: u32,             // result in percentage
    timestamp: u64,          // timestamp of the result
    game_difficulty: String, // difficulty of the game (easy, medium, hard)
    game_content: String,    // content of the game (the question, the text, etc)
}

// on reçois les résultats gloabaux des jeux du user pour un cours donné, ainsi on sait un peu déjà ce que le joueur gérère
// le mieux dans le cours
struct UserCourseResult {
    course_id: u32,                            // course id
    course_subject: String,                    // subject of the course
    course_keywords: Vec<String>,              // keywords of the course
    course: String,                            // change later by course object
    course_games_results: Vec<UserGameResult>, // results of the games for this course
}

// contenu d'un mode learn
struct LearnModeContent {
    synthesis_text: String,    // text to learn
    games: Vec<LearnModeGame>, // games to play
}

// resultat d'un round de mode learn
struct LearnModeRoundResult {
    user_id: u32,                              // user id
    course_id: u32,                            // course id
    round_id: u32,                             // round id
    games_results: Vec<UserGameResult>,        // results of the games in this round
    overall_result: u32,                       // overall result in percentage
    timestamp: u64,                            // timestamp of the round completion
    learn_mode_content: Vec<LearnModeContent>, // content used in this round
}
