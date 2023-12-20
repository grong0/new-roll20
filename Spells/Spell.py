from scrapy import Selector  # type: ignore
from scrapy.http import Response

from Component import Component  # type: ignore


class Spell:
    def __init__(self, response: Response):
        name = response.css("div.page-title.page-header").css("span::text").get()
        if name == None:
            name = ""
        self.name: str = name

        content = response.css("div.main-content")
        critical_role_data = content.css("p::text").get()
        critical_role_mod = 0
        if critical_role_data != None and "Critical Role" in critical_role_data:
            critical_role_mod = 1

        source = (
            content.css("p::text").get() if critical_role_mod == 0 else "Critical Role"
        )
        if source != None:
            self.source: str = source[8:]
        else:
            self.source: str = ""

        schData = content.css("em::text").get()
        school = ""
        level = ""
        ritual = False
        if schData != None:
            schData = schData.split(" ")

            school = (
                schData[0]
                if "level" not in schData[0]
                else schData[1][0].upper() + schData[1][1:]
            )
            level = (
                schData[1]
                if "level" not in schData[0]
                else schData[0][: schData[0].find("-")]
            )
            if len(schData) == 3:
                ritual = True
        self.school: str = school
        self.level: str = level
        self.ritual: bool = ritual

        casting_time = content.css("p::text").getall()[1 + critical_role_mod][1:]
        self.casting_time: str = casting_time

        spell_range = content.css("p::text").getall()[3 + critical_role_mod][1:]
        self.spell_range: str = spell_range

        component_data = content.css("p::text").getall()[5 + critical_role_mod][1:]
        self.components = Component(component_data)

        duration = content.css("p::text").getall()[7 + critical_role_mod][1:]
        self.duration: str = duration

        desc_up = content.css("p::text").getall()[
            8
            + critical_role_mod : len(content.css("p::text").getall())
            - len(content.css("p").css("a::text").getall())
        ]
        description = desc_up
        upcast = ""
        if desc_up[-1][0] == " ":
            description = desc_up[: len(desc_up) - 1]
            upcast = desc_up[-1][1:]
        self.description: list[str] = description
        self.upcast: str = upcast

        desc_list: list[str] = []
        for x in content.css("ul"):  # type: ignore
            desc_italic = x.css("em::text").get()
            desc_text = x.css("li::text").get()
            if desc_italic != None:
                desc_list.append(desc_italic)
            elif desc_text != None:
                desc_list.append(desc_text)
        self.desc_list: list[str] = desc_list

        spell_lists = content.css("p").css("a::text").getall()
        self.spell_lists: list[str] = spell_lists

    def get_as_dict(self) -> dict[str, str | list[str] | bool | dict[str, bool | str]]:
        return {
            "name": self.name,
            "source": self.source,
            "school": self.school,
            "level": self.level,
            "ritual": self.ritual,
            "casting_time": self.casting_time,
            "spell_range": self.spell_range,
            "components": self.components.get_as_dict(),
            "duration": self.duration,
            "description": self.description,
            "desc_list": self.desc_list,
            "upcast": self.upcast,
            "spell_lists": self.spell_lists,
        }
