from scrapy import Selector  # type: ignore


class Component:
    def __init__(self, content: str):
        materials = content[content.find("(") : content.find(")")][1:]
        components = content[: content.find(" (")]
        self.verbal = True if "V" in components else False
        self.somatic = True if "S" in components else False
        self.material = True if materials != "" else False
        self.items = materials

    def get_as_dict(self) -> dict[str, bool | str]:
        return {
            "verbal": self.verbal,
            "somatic": self.somatic,
            "material": self.material,
            "items": self.items,
        }
