from scrapy import Spider, Selector, Request
from scrapy.http import Response
from RaceFunctions import get_race_table_info, to_list


class RaceSpider(Spider):
    name = "RaceSpider"
    start_urls = ["http://dnd5e.wikidot.com/"]
    race_urls = []
    finished_races = []
    new_loop = False
    lineages = []
    count = 0
    done = False
    lineagesDone = []

    def parse(self, response: Response):
        if self.new_loop == False:
            page = response.css("div#page-content")

            for index, x in enumerate(page.css("h1")):
                name = x.css("span::text").get()
                table, links = get_race_table_info(
                    page.css("table.wiki-content-table")[index]
                )
                if name == "Unearthed Arcana":
                    table, links = get_race_table_info(
                        page.css("table.wiki-content-table")[-1]
                    )
                    races = []
                    for row in table:
                        for col in row:
                            if col["link"] != None:
                                races.append(col)
                            else:
                                races[-1]["source"] = col["name"]
                else:
                    races = to_list(table)

                for link in links:
                    self.race_urls.append(link)

                if name == "Setting Specific Lineages":
                    self.lineages.append({"name": name})
                else:
                    self.lineages.append({"name": name, "races": races})

            settingSpecificLineages = []
            for index, x in enumerate(page.css("h2")):
                name = x.css("span::text").get()
                table, links = get_race_table_info(
                    page.css("table.wiki-content-table")[index + 4]
                )
                for link in links:
                    self.race_urls.append(link)
                races = to_list(table)
                settingSpecificLineages.append({"name": name, "races": races})
            self.lineages[-2]["lineages"] = settingSpecificLineages

            self.new_loop = True
            yield {"race_urls": self.race_urls}

        else:
            pass

        for _, x in enumerate(self.race_urls):
            yield (Request(x, callback=self.parse))

        if self.done == True:
            for lineage in self.lineages:
                if lineage["name"] not in self.lineagesDone:
                    self.lineagesDone.append(lineage["name"])
                    yield lineage
