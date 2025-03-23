import json
import datetime
from typing import Any

class Class:
    name: str
    source: str
    page: int
    srd: bool
    basic_rules: bool

    def __init__(self, object: dict[str, Any]) -> None:
        self.name = object['name'] if 'name' in object.keys() else ""
        self.source = object['source'] if 'source' in object.keys() else ""
        self.page = object['page'] if 'page' in object.keys() else -1
        self.srd = object['srd'] if 'srd' in object.keys() else False
        self.basicRules = object['basicRules'] if 'basicRules' in object.keys() else ""

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        return copy

class Classs:
    classes: dict[str, Class]

    def __init__(self) -> None:
        with open("data/raw/classes.json") as f:
            obj = json.load(f)

            self.classes = {}
            for class_name in obj['class_name']:
                self.classes[class_name['name']] = Class(class_name)

    def as_dict(self) -> dict[str, Any]:
        """This is effectivaly read only as it creates a copy

        Returns:
            dict[str, Any]: the class as a dict
        """

        copy = self.__dict__.copy()
        for class_name_name in self.classes.keys():
            copy['classes'][class_name_name] = self.classes[class_name_name].as_dict()
        return copy

if __name__ == "__main__":
    times = []
    count = 1000
    for _ in range(0, count):
        start = datetime.datetime.now()
        classes = Classs()
        end = datetime.datetime.now()
        times.append((end - start).microseconds)
        del classes

    average_time = sum(times) / count

    print(f"average time was {average_time} microseconds")
