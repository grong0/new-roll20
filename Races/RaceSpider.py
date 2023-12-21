from scrapy import Spider, Selector, Request
from scrapy.http import Response
from scrapy.crawler import CrawlerProcess
from RaceFunctions import get_lineage_table_info, array_2D_to_1D
from Lineage import Lineage
from Race import Race
from prettyprinter import pprint
from Lineages import Lineages


class RaceSpider(Spider):
    name = "RaceSpider"
    start_urls = ["http://dnd5e.wikidot.com/lineage"]
    race_urls = []
    lineages: Lineages = Lineages()
    new_loop = False

    finished_races = []
    count = 0
    done = False
    lineages_done = []

    def parse(self, response: Response):
        print("\n\n\n\n\n")
        if self.new_loop == False:
            page = response.css("div#page-content")

            # Getting Normal Lineages and Their Races
            for index, header in enumerate(page.css("h1")):
                name = header.css("span::text").get()
                if name == "Setting Specific Lineages" or name == "Unearthed Arcana":
                    continue
                table, links = get_lineage_table_info(
                    page.css("table.wiki-content-table")[index]
                )
                self.race_urls.extend(links)
                races = array_2D_to_1D(table)
                self.lineages.add_lineage(Lineage(name, False, races))

            # Getting Setting Specific Lineages and Their Races
            for index, sub_header in enumerate(page.css("h2")):
                name = sub_header.css("span::text").get()
                table, links = get_lineage_table_info(
                    page.css("table.wiki-content-table")[index + 4]
                )
                self.race_urls.extend(links)
                races = array_2D_to_1D(table)
                self.lineages.add_lineage(Lineage(name, True, races))

            # Getting Unearthed Arcana Lineages and Their Races
            table, links = get_lineage_table_info(
                page.css("table.wiki-content-table")[-1]
            )
            races = []
            for row in table:
                for col in row:
                    if col["link"] != None:
                        races.append(col)
                        self.race_urls.append(col["link"])
                    else:
                        races[-1]["source"] = col["name"]
            self.lineages.add_lineage(Lineage("Unearthed Arcana", False, races))

            pprint(self.lineages.get_as_dict())

            self.new_loop = True
            yield {"race_urls": self.race_urls}

        else:
            pass
            # race = Race(response)

            # for lineage in self.lineages:

        # for _, x in enumerate(self.race_urls):
        #     yield (Request(x, callback=self.parse))

        print("\n\n\n\n\n")

        # if self.done == True:
        #     for lineage in self.lineages:
        #         if lineage["name"] not in self.lineages_done:
        #             self.lineages_done.append(lineage["name"])
        #             yield lineage


RacesProcess = CrawlerProcess(
    settings={"FEEDS": {"races.json": {"format": "json", "overwrite": True}}}
)
RacesProcess.crawl(RaceSpider)
RacesProcess.start()
