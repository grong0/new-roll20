from Race import Race
from Proficiency import Proficiency

class Trait:
    name: str
    races: list[Race]
    subraces: list[Race]
    parent: "Trait"
    description: list[str]
    proficiencies: list[Proficiency]
    trait_specific: any
    
    def __init__(self) -> None:
        pass