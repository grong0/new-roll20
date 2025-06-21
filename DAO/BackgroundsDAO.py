import json
import datetime
from typing import Any
from src.DAO.MiscClasses import (
    SkillProficiencies,
    LanguageProficiencies,
    StartingEquipment,
    Source,
    ToolProficiencies,
    AdditionalSpells,
    Prerequisite,
    SkillToolLanguageProficiencies,
)


class Background:
    """
    Has a _copy
    """

    name: str
    source: str
    page: int
    srd: bool
    basic_rules: bool
    skill_proficiencies: SkillProficiencies
    language_proficiencies: LanguageProficiencies
    starting_equipment: StartingEquipment
    entries: list
    has_fluff: bool
    tool_proficiencies: ToolProficiencies
    feats: list[str]
    from_feature: list[str]
    has_fluff_images: bool
    additional_spells: AdditionalSpells
    additional_sources: list[Source]
    prerequisite: Prerequisite
    skill_tool_language_proficiencies: SkillToolLanguageProficiencies
    other_sources: list[Source]

    def __init__(self, object: dict[str, Any]) -> None:
        self.name = object["name"] if "name" in object.keys() else ""
        self.source = object["source"] if "source" in object.keys() else ""
        self.page = object["page"] if "page" in object.keys() else -1
        self.srd = object["srd"] if "srd" in object.keys() else False
        self.basicRules = object["basicRules"] if "basicRules" in object.keys() else ""
        self.skill_proficiencies = (
            SkillProficiencies(object["skillProficiencies"][0])
            if "skillProficiencies" in object.keys()
            else SkillProficiencies(None)
        )
        self.language_proficiencies = (
            LanguageProficiencies(object["languageProficiencies"][0])
            if "languageProficiencies" in object.keys()
            else LanguageProficiencies(None)
        )
        self.starting_equipment = (
            StartingEquipment(object["startingEquipment"])
            if "startingEquipment" in object.keys()
            else StartingEquipment([])
        )
        self.entries = object["entries"] if "entries" in object.keys() else []
        self.has_fluff = object["hasFluff"] if "hasFluff" in object.keys() else False
        self.tool_proficiencies = (
            ToolProficiencies(object["toolProficiencies"])
            if "toolProficiencies" in object.keys()
            else ToolProficiencies(None)
        )
        self.feats = []
        if 'feats' in object.keys():
            for feat_obj in object['feats']:
                self.feats.extend([str(key) for key in feat_obj.keys()])
        self.from_feature = [str(key) for key in object["fromFeature"].keys()] if "fromFeature" in object.keys() else []
        self.has_fluff_images = object["hasFluffImages"] if "hasFluffImages" in object.keys() else False
        self.additional_spells = (
            AdditionalSpells(object["additionalSpells"])
            if "additionalSpells" in object.keys()
            else AdditionalSpells(None)
        )
        self.additional_sources = (
            [Source(source) for source in object["additionalSources"]] if "additionalSources" in object.keys() else []
        )
        self.prerequisite = (
            Prerequisite(object["prerequisite"]) if "prerequisite" in object.keys() else Prerequisite(None)
        )
        self.skill_tool_language_proficiencies = (
            SkillToolLanguageProficiencies(object["skillToolLanguageProficiencies"])
            if "skillToolLanguageProficiencies" in object.keys()
            else SkillToolLanguageProficiencies(None)
        )
        self.other_sources = (
            [Source(source) for source in object["otherSources"]] if "otherSources" in object.keys() else []
        )

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        copy["skill_proficiencies"] = self.skill_proficiencies.__dict__
        copy["language_proficiencies"] = self.language_proficiencies.__dict__
        copy["starting_equipment"] = self.starting_equipment.as_dict()
        copy["tool_proficiencies"] = self.tool_proficiencies.__dict__
        copy["additional_spells"] = self.additional_spells.__dict__
        copy["additional_sources"] = [source.__dict__ for source in self.additional_sources]
        copy["prerequisite"] = self.prerequisite.as_dict()
        copy["skill_tool_language_proficiencies"] = self.skill_tool_language_proficiencies.as_dict()
        copy["other_sources"] = [source.__dict__ for source in self.other_sources]
        return copy


class Backgrounds:
    backgrounds: dict[str, Background]

    def __init__(self) -> None:
        with open("data/raw/backgrounds.json") as f:
            obj = json.load(f)

            self.backgrounds = {}
            for background in obj["background"]:
                self.backgrounds[background["name"]] = Background(background)

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        for background_name in self.backgrounds.keys():
            copy["backgrounds"][background_name] = self.backgrounds[background_name].as_dict()
        return copy


if __name__ == "__main__":
    # times = []
    # count = 1000
    # for _ in range(0, count):
    #     start = datetime.datetime.now()
    #     backgrounds = Backgrounds()
    #     end = datetime.datetime.now()
    #     times.append((end - start).microseconds)
    #     del backgrounds

    # average_time = sum(times) / count

    # print(f"average time was {average_time} microseconds")
    backgrounds = Backgrounds()
    print(json.dumps(backgrounds.as_dict(), indent=4))
