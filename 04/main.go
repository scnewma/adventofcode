package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"sort"
	"strconv"
	"time"
)

const (
	timeLayout = "2006-01-02 15:04"
)

func main() {
	f, err := os.Open("events.txt")
	if err != nil {
		log.Fatal(err)
	}

	scanner := bufio.NewScanner(f)
	var events []*Event
	for scanner.Scan() {
		e := newEvent(scanner.Text())
		events = append(events, &e)
	}

	events = createTimeline(events)
	gSleepTimes := guardSleepTimes(events)
	gid, maxMinute := mostMinutesAsleep(gSleepTimes)
	fmt.Printf("Guard #%d slept the most overall. Most often during minute: %d -> %d\n", gid, maxMinute, gid*maxMinute)
	gid, maxMinute = mostFrequentMinuteAsleep(gSleepTimes)
	fmt.Printf("Guard #%d slept the most frequently during minute %d -> %d\n", gid, maxMinute, gid*maxMinute)
}

type EventType int

const (
	Wake EventType = iota
	Sleep
	BeginShift
)

type Event struct {
	Type  EventType
	Guard int
	Time  time.Time
}

var eventRegexp = regexp.MustCompile("^[\\[](.*)[\\]](?: Guard #(\\d+))? ([a-z ]+)$")
var etLookup = map[string]EventType{
	"wakes up":     Wake,
	"falls asleep": Sleep,
	"begins shift": BeginShift,
}

func newEvent(e string) Event {
	matches := eventRegexp.FindStringSubmatch(string(e))
	ts, gid, ev := matches[1], matches[2], matches[3]

	t, err := time.Parse(timeLayout, ts)
	if err != nil {
		log.Fatal(err)
	}

	return Event{Type: etLookup[ev], Time: t, Guard: mustAtoi(gid)}
}

func createTimeline(events []*Event) []*Event {
	sort.Slice(events, func(i, j int) bool {
		return events[i].Time.Before(events[j].Time)
	})

	currentGuard := -1
	for i := 0; i < len(events); i++ {
		e := events[i]
		if e.Guard == 0 {
			e.Guard = currentGuard
		} else {
			currentGuard = e.Guard
		}

	}

	return events
}

func guardSleepTimes(events []*Event) map[int][]int {
	guardSleep := make(map[int][]int)

	currentGuard := -1
	var lastSleep time.Time
	for _, e := range events {
		switch e.Type {
		case Sleep:
			lastSleep = e.Time
		case Wake:
			// increment sleep time for each minute asleep
			for i := lastSleep.Minute(); i < e.Time.Minute(); i++ {
				guardSleep[currentGuard][i]++
			}
		default:
			currentGuard = e.Guard
			if _, ok := guardSleep[currentGuard]; !ok {
				guardSleep[currentGuard] = make([]int, 60)
			}
		}
	}

	return guardSleep
}

func mostMinutesAsleep(gSleepTimes map[int][]int) (int, int) {
	maxGid := -1
	maxSleepMinute := 0
	maxSleep := 0
	for gid, minutes := range gSleepTimes {
		gSleepTotal := 0
		gMaxSleepMinute := 0
		gMaxSleep := 0
		for minute, minuteSleep := range minutes {
			gSleepTotal += minuteSleep

			if minuteSleep > gMaxSleep {
				gMaxSleepMinute = minute
				gMaxSleep = minuteSleep
			}
		}
		if gSleepTotal > maxSleep {
			maxSleep = gSleepTotal
			maxGid = gid
			maxSleepMinute = gMaxSleepMinute
		}
	}

	return maxGid, maxSleepMinute
}

func mostFrequentMinuteAsleep(gSleepTimes map[int][]int) (int, int) {
	sleepMax := 0
	maxGid := 0
	maxMinute := 0
	for gid, minutes := range gSleepTimes {
		for minute, minuteSleep := range minutes {
			if minuteSleep > sleepMax {
				sleepMax = minuteSleep
				maxGid = gid
				maxMinute = minute
			}
		}
	}
	return maxGid, maxMinute
}

func mustAtoi(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		return 0
	}
	return i
}
