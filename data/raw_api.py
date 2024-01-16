import json
from typing import Optional


class API:
    races: list[dict]
    subraces: list[dict]
    books: list[dict]

    def __init__(self) -> None:
        with open("./data/raw/races.json") as f:
            object = json.load(f)
            self.races = object["race"]
            self.subraces = object["subrace"]
        with open("./data/raw/books.json") as f:
            object = json.load(f)
            self.books = object["book"]

    def get_unique_race_names(self) -> list[str]:
        marked_races = []
        for race in self.races:
            processed_name = race["name"].split("(")[0].strip()
            if processed_name not in marked_races:
                marked_races.append(processed_name)
        return marked_races

    def get_race(self, name: str, source: str) -> Optional[dict]:
        for race in self.races:
            if race["name"] == name and race["source"] == source:
                return race
        return None

    def get_races(self, name: str) -> list[dict]:
        races = []
        for race in self.races:
            processed_name = race["name"].split("(")[0].strip()
            # print(f"from {race['name']} to {processed_name}")
            if processed_name == name:
                races.append(race)
        return races

    def get_race_subraces(self, race: dict) -> list[dict]:
        subraces = []
        for subrace in self.subraces:
            if (
                subrace["raceName"] == race["name"]
                and subrace["raceSource"] == race["source"]
            ):
                subraces.append(subrace)
        return subraces

    def get_race_entries(self, race: dict) -> list[dict]:
        if "entries" in race.keys():
            return race["entries"]
        elif "_copy" in race.keys():
            copy_race = self.get_race(race["_copy"]["name"], race["_copy"]["source"])
            entries = self.get_race_entries(copy_race).copy()
            for entry in race["_copy"]["_mod"]["entries"]:
                match entry["mode"]:
                    case "appendArr":
                        for existing_entry in entries:
                            if existing_entry["name"] == entry["items"]["name"]:
                                existing_entry["entries"].extend(
                                    entry["items"]["entries"]
                                )
                        entries.append(entry["items"])
                    case "replaceArr":
                        for existing_entry in entries:
                            if existing_entry["name"] == entry["items"]["name"]:
                                existing_entry = entry["items"]
                    case _:
                        print(
                            "copy mode was something other than appendArr and replaceArr"
                        )
            return entries
        return [{"name": "error", "entries": ["something went wrong"]}]

    def get_race_traitTags(self, race: dict) -> list[str]:
        if "traitTags" not in race.keys() and "_copy" in race.keys():
            copy_race = self.get_race(race["_copy"]["name"], race["_copy"]["source"])
            return self.get_race_traitTags(copy_race)
        elif "traitTags" in race.keys():
            return race["traitTags"] if race["traitTags"] is not None else []
        else:
            return []

    def get_race_ability(self, race: dict) -> list[dict[str, int]]:
        if "ability" not in race.keys() and "_copy" in race.keys():
            copy_race = self.get_race(race["_copy"]["name"], race["_copy"]["source"])
            return copy_race["ability"]
        elif "ability" in race.keys():
            return race["ability"]
        else:
            return [{
                "Err": 0
            }]

    def get_race_speed(self, race: dict) -> dict[str, int] | int:
        if "speed" not in race.keys() and "_copy" in race.keys():
            copy_race = self.get_race(race["_copy"]["name"], race["_copy"]["source"])
            return copy_race["speed"]
        elif "speed" in race.keys():
            return race["speed"]
        else:
            return -1

    def get_race_size(self, race: dict) -> list[str]:
        if "size" not in race.keys() and "_copy" in race.keys():
            copy_race = self.get_race(race["_copy"]["name"], race["_copy"]["source"])
            return copy_race["size"]
        elif "size" in race.keys():
            return race["size"]
        else:
            return ["Err"]

    def race_has_base(self, race: dict) -> bool:
        for subrace in self.get_race_subraces(race):
            if "name" not in subrace.keys():
                return True
        return False

    def race_has_subraces(self, race: dict) -> bool:
        for subrace in self.subraces:
            if (
                subrace["raceName"] == race["name"]
                and subrace["raceSource"] == race["source"]
            ):
                return True
        return False

    def get_subrace(self, name: str, source: str) -> Optional[dict]:
        for subrace in self.subraces:
            if (
                "name" in subrace.keys()
                and subrace["name"] == name
                and subrace["source"] == source
            ):
                return subrace
        return None

    def get_subrace_traitTags(self, subrace) -> dict:
        if "overwrite" in subrace.keys() and "traitTags" in subrace["overwrite"]:
            if "traitTags" in subrace.keys():
                return subrace["traitTags"] if subrace["traitTags"] is not None else []
            else:
                return []
        return self.get_race_traitTags(self.get_subrace_parent(subrace))

    def get_subrace_entries(self, subrace: dict) -> list[dict]:
        entries: list[dict] = self.get_subrace_parent(subrace)["entries"].copy()
        for subrace_entry in subrace["entries"]:
            if "data" not in subrace_entry.keys():
                entries.append(subrace_entry)
            elif "overwrite" in subrace_entry["data"].keys():
                found = False
                for index, entry in enumerate(entries):
                    if entry["name"] == subrace_entry["data"]["overwrite"]:
                        found = True
                        entries[index] = subrace_entry
                        break
                if not found:
                    entries.append(subrace_entry)
        return entries

    def get_subrace_parent(self, subrace: dict) -> dict:
        return self.get_race(subrace["raceName"], subrace["raceSource"])

    def get_subrace_ability(self, subrace: dict) -> list[dict]:
        if "overwrite" in subrace.keys() and "ability" in subrace["overwrite"]:
            return subrace["ability"]
        else:
            print("subrace didn't have ability, checking parent...")
            return self.get_race_ability(self.get_subrace_parent(subrace))

    def get_book_from_source(self, source: str) -> dict:
        for book in self.books:
            if book["source"] == source:
                return book


if __name__ == "__main__":
    api = API()
    race = api.get_race("Dragonborn", "PHB")
    subraces = api.get_race_subraces(race)
    print(subraces)
    # subrace = api.get_subrace("Ravenite", "EGW")
    # parent_race = api.get_subrace_parent(subrace)
