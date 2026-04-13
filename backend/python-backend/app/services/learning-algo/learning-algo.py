# algo de apprentissage 

"""
on doit pouvoir : 
1 - pendant que un user fait des exo decider quel exo prochain on vas lui proposer 
2 - on doit pouvoir lui recommander quel sujet il doit etudier 
3 - on doit savoir quel exo est le plus adapte pour lui 
--> on check ses reusltats et on voit avec quel exo il a le plus progresse 


 a chaque fois que un user lance un exo on doit le store dnas une variable ou quoi 
 a la fin de son exo on voit ou il a fait des erreurs , et on stock ca dans la db 


 
"""

class learning_algoStructure:
    
    exo_id = None
    user_id = None
    score = None
    date = None
    course_id = None
    game_id = None
    game_difficulty = None
    
    def __init__(self, name):
        self.name = name
    

class LearningAlgo:
    def __init__(self, name):
        self.name = name

    
    
    def get_user_res(self, user_id):
        # code pour récupérer les résultats de l'utilisateur
        pass

