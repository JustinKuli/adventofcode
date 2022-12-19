package day19

import (
	"bufio"
	"fmt"
	"log"
	"strings"

	"github.com/JustinKuli/aoc2022/aoc"
)

type resources struct {
	ore      int
	clay     int
	obsidian int
	geode    int
}

func (rc resources) String() string {
	return fmt.Sprintf("{%v ore, %v clay, %v obsidian, %v geode}", rc.ore, rc.clay, rc.obsidian, rc.geode)
}

func (rc resources) ser() string {
	return fmt.Sprintf("%x,%x,%x,%x", rc.ore, rc.clay, rc.obsidian, rc.geode) // hex to possibly save space
}

func (rc1 resources) gteDiff(rc2 resources) (bool, resources) {
	ore := rc1.ore - rc2.ore
	if ore < 0 {
		return false, resources{}
	}

	clay := rc1.clay - rc2.clay
	if clay < 0 {
		return false, resources{}
	}

	obsidian := rc1.obsidian - rc2.obsidian
	if obsidian < 0 {
		return false, resources{}
	}

	geode := rc1.geode - rc2.geode
	if geode < 0 {
		return false, resources{}
	}

	return true, resources{ore: ore, clay: clay, obsidian: obsidian, geode: geode}
}

type blueprint struct {
	id          int
	oreBot      resources
	clayBot     resources
	obsidianBot resources
	geodeBot    resources
	// maxOreCost      int
	// maxClayCost     int
	// maxObsidianCost int
}

func bpFromStr(s string) blueprint {
	s = strings.TrimPrefix(s, "Blueprint ")
	idStr, s := aoc.MustCut(s, ": Each ore robot costs ")
	id := aoc.MustInt(idStr)

	oreBotCostStr, s := aoc.MustCut(s, " ore. Each clay robot costs ")
	orebot := resources{ore: aoc.MustInt(oreBotCostStr)}

	clayBotCostStr, s := aoc.MustCut(s, " ore. Each obsidian robot costs ")
	claybot := resources{ore: aoc.MustInt(clayBotCostStr)}

	obsidianBotCost1Str, s := aoc.MustCut(s, " ore and ")
	obsidianBotCost2Str, s := aoc.MustCut(s, " clay. Each geode robot costs ")
	obsidianbot := resources{ore: aoc.MustInt(obsidianBotCost1Str), clay: aoc.MustInt(obsidianBotCost2Str)}

	geodeBotCost1Str, s := aoc.MustCut(s, " ore and ")
	geodeBotCost2Str := strings.TrimSuffix(s, " obsidian.")
	geodebot := resources{ore: aoc.MustInt(geodeBotCost1Str), obsidian: aoc.MustInt(geodeBotCost2Str)}

	return blueprint{
		id:          id,
		oreBot:      orebot,
		clayBot:     claybot,
		obsidianBot: obsidianbot,
		geodeBot:    geodebot,
	}
}

func (bp blueprint) String() string {
	return fmt.Sprintf("%v: oreBot=%v, clayBot=%v, obsBot=%v, geodeBot=%v",
		bp.id, bp.oreBot, bp.clayBot, bp.obsidianBot, bp.geodeBot)
}

func Run(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()

	log.Println("start")

	fs := bufio.NewScanner(f)
	totalQuality := 0
	top3prod := 1
	dopart2 := false
	for fs.Scan() {
		bp := bpFromStr(fs.Text())
		memos := make(map[string]int)
		bk := 0
		bestKnown := &bk
		quality := memoWBMaxGeode(&bp, resources{ore: 1}, resources{}, 24, memos, bestKnown)
		log.Println(bp.id, quality)

		totalQuality += bp.id * quality

		if bp.id <= 3 && dopart2 {
			log.Println(bp.id, "part 2 start")
			memos2 := make(map[string]int)
			bk2 := 0
			bestKnown2 := &bk2
			quality2 := memoWBMaxGeode(&bp, resources{ore: 1}, resources{}, 32, memos2, bestKnown2)
			log.Println(bp.id, "part 2", quality2)
			top3prod *= quality2
		}
	}

	fmt.Printf("%v - part one: %v\n", title, totalQuality)
	fmt.Printf("%v - part two: %v\n", title, top3prod)
}

func memoWBMaxGeode(bp *blueprint, bots, acc resources, timeLeft int, memos map[string]int, bestKnown *int) int {
	if timeLeft == 1 {
		ans := acc.geode + bots.geode
		if ans > *bestKnown {
			*bestKnown = ans
		}
		return ans
	}

	// if we were able to build a geode bot on all the remaining turns, this would be the total geodes gathered
	theoryBest := bots.geode*timeLeft + triangle(timeLeft+1)
	if theoryBest < *bestKnown {
		return theoryBest
	}

	mem := fmt.Sprintf("%v:%v:%x", bots.ser(), acc.ser(), timeLeft)
	if ans, found := memos[mem]; found {
		return ans
	}

	// The "do nothing" option
	best := memoWBMaxHelp(bp, bots, acc, resources{}, timeLeft, memos, bestKnown)

	// check build possibilities
	canOre, afterOre := acc.gteDiff(bp.oreBot)
	if canOre {
		oreAns := memoWBMaxHelp(bp, bots, afterOre, resources{ore: 1}, timeLeft, memos, bestKnown)
		if oreAns > best {
			best = oreAns
		}
	}

	canClay, afterClay := acc.gteDiff(bp.clayBot)
	if canClay {
		clayAns := memoWBMaxHelp(bp, bots, afterClay, resources{clay: 1}, timeLeft, memos, bestKnown)
		if clayAns > best {
			best = clayAns
		}
	}

	canObsidian, afterObsidian := acc.gteDiff(bp.obsidianBot)
	if canObsidian {
		obsidianAns := memoWBMaxHelp(bp, bots, afterObsidian, resources{obsidian: 1}, timeLeft, memos, bestKnown)
		if obsidianAns > best {
			best = obsidianAns
		}
	}

	canGeode, afterGeode := acc.gteDiff(bp.geodeBot)
	if canGeode {
		geodeAns := memoWBMaxHelp(bp, bots, afterGeode, resources{geode: 1}, timeLeft, memos, bestKnown)
		if geodeAns > best {
			best = geodeAns
		}
	}

	memos[mem] = best
	return best
}

func memoWBMaxHelp(bp *blueprint, bots, acc, newbot resources, timeLeft int, memos map[string]int, bestKnown *int) int {
	// add resources from existing bots
	acc.ore += bots.ore
	acc.clay += bots.clay
	acc.obsidian += bots.obsidian
	acc.geode += bots.geode

	// add built bot (if applicable)
	bots.ore += newbot.ore
	bots.clay += newbot.clay
	bots.obsidian += newbot.obsidian
	bots.geode += newbot.geode

	// profit?
	return memoWBMaxGeode(bp, bots, acc, timeLeft-1, memos, bestKnown)
}

var (
	maxtri = 40
	tri    []int
)

func triangle(i int) int {
	if len(tri) == 0 {
		tri = make([]int, maxtri)
		for x := 1; x < maxtri; x++ {
			tri[x] = tri[x-1] + x
		}
	}

	return tri[i]
}
