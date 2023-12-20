class Race:
    def __init__(self) -> None:
        self.name = "Name"
        self.description = ["Description"]
    
    def get_as_dict(self) -> dict[str, str | list[str]]:
        return {
            "name": self.name,
            "description": self.description
        }
