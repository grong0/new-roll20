from scrapy.http import Response


class Race:
    name: str
    link: str
    source: str
    description: str
    feats: list[str]
    sub_feats: list[dict[str, str | int]] # "text": "", "index" 0, "depth": 0
    sub_races: list["Race"]
    
    def __init__(self, name: str, link: str) -> None:
        self.name = name
        self.link = link
        self.source = ""

    def compile(self, response: Response) -> None:
        pass
    
    def get_as_dict(self) -> dict[str, str | list[str]]:
        return {
            "name": self.name,
            "link": self.link,
            "source": self.source
            # "description": self.description,
            # "feats": self.feats,
            # "sub_feats": self.sub_feats,
            # "sub_races": self.sub_races,
        }
