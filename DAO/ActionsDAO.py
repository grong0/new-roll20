import json
import datetime
from typing import Any
from src.DAO.MiscClasses import Time

class Action:
    name: str
    source: str
    page: int
    srd: bool
    basic_rules: bool
    time: Time
    entries: list[dict[str, list[str] | str]]
    see_also_action: list[str]
    from_variant: str

    def __init__(self, object: dict[str, Any]) -> None:
        self.name = object['name'] if 'name' in object.keys() else ""
        self.source = object['source'] if 'source' in object.keys() else ""
        self.page = object['page'] if 'page' in object.keys() else -1
        self.srd = object['srd'] if 'srd' in object.keys() else False
        self.basic_rules = object['basicRules'] if 'basicRules' in object.keys() else False
        self.time = object['time'] if 'time' in object.keys() else Time(None)
        self.entries = object['entries'] if 'entries' in object.keys() else []
        self.see_also_action = object['seeAlsoAction'] if 'seeAlsoAction' in object.keys() else []
        self.from_variant = object['fromVariant'] if 'fromVariant' in object.keys() else ""

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        copy['time'] = self.time.__dict__
        return copy

class Actions:
    actions: dict[str, Action]

    def __init__(self) -> None:
        with open("data/raw/actions.json") as f:
            obj = json.load(f)

            self.actions = {}
            for action in obj['action']:
                self.actions[action['name']] = Action(action)

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        for action_name in self.actions.keys():
            copy['actions'][action_name] = self.actions[action_name].as_dict()
        return copy

if __name__ == "__main__":
    times = []
    count = 1000
    for _ in range(0, count):
        start = datetime.datetime.now()
        actions = Actions()
        end = datetime.datetime.now()
        times.append((end - start).microseconds)
        del actions

    average_time = sum(times) / count

    print(f"average time was {average_time} microseconds")
