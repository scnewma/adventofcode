package main

import (
	"reflect"
	"testing"
	"time"
)

func TestNewEvent(t *testing.T) {
	tests := []struct {
		in  string
		out Event
	}{
		{"[1518-05-12 00:39] wakes up", Event{Type: Wake, Time: time.Date(1518, time.May, 12, 0, 39, 0, 0, time.UTC)}},
		{"[1518-05-09 00:21] falls asleep", Event{Type: Sleep, Time: time.Date(1518, time.May, 9, 0, 21, 0, 0, time.UTC)}},
		{"[1518-02-06 23:51] Guard #2081 begins shift", Event{Type: BeginShift, Guard: 2081, Time: time.Date(1518, time.February, 6, 23, 51, 0, 0, time.UTC)}},
	}

	for _, tt := range tests {
		t.Run(tt.in, func(t *testing.T) {
			got := newEvent(tt.in)
			if !reflect.DeepEqual(got, tt.out) {
				t.Errorf("expected: %v, got: %v", tt.out, got)
			}
		})
	}
}

func TestGuardSleepTimes(t *testing.T) {
	events := []*Event{
		&Event{Guard: 617, Type: BeginShift, Time: time.Date(1518, time.January, 30, 0, 0, 0, 0, time.UTC)},
		&Event{Guard: 617, Type: Sleep, Time: time.Date(1518, time.January, 30, 0, 42, 0, 0, time.UTC)},
		&Event{Guard: 617, Type: Wake, Time: time.Date(1518, time.January, 30, 0, 53, 0, 0, time.UTC)},
		&Event{Guard: 617, Type: Sleep, Time: time.Date(1518, time.January, 30, 0, 58, 0, 0, time.UTC)},
		&Event{Guard: 1201, Type: BeginShift, Time: time.Date(1518, time.January, 31, 0, 2, 0, 0, time.UTC)},
		&Event{Guard: 617, Type: BeginShift, Time: time.Date(1518, time.February, 23, 23, 59, 0, 0, time.UTC)},
		&Event{Guard: 617, Type: Wake, Time: time.Date(1518, time.February, 24, 00, 07, 0, 0, time.UTC)},
		&Event{Guard: 617, Type: Sleep, Time: time.Date(1518, time.February, 24, 00, 42, 0, 0, time.UTC)},
		&Event{Guard: 617, Type: Wake, Time: time.Date(1518, time.February, 24, 00, 52, 0, 0, time.UTC)},
		&Event{Guard: 617, Type: Sleep, Time: time.Date(1518, time.February, 24, 00, 53, 0, 0, time.UTC)},
		&Event{Guard: 617, Type: Wake, Time: time.Date(1518, time.February, 24, 00, 56, 0, 0, time.UTC)},
		&Event{Guard: 617, Type: Sleep, Time: time.Date(1518, time.February, 24, 00, 59, 0, 0, time.UTC)},
	}

	gst := guardSleepTimes(events)
	expectedSleepMinutes := make([]int, 60)
	for _, m := range []int{42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 53, 54, 55} {
		expectedSleepMinutes[m]++
	}
	if !reflect.DeepEqual(gst[617], expectedSleepMinutes) {
		t.Errorf("expected guard 617 to have slept %v, got %v", expectedSleepMinutes, gst[617])
	}
}

func TestMostMinutesAsleep(t *testing.T) {
	gst := make(map[int][]int)
	gst[0] = make([]int, 60)
	gst[0][10] = 5
	gst[0][5] = 2
	gst[0][2] = 1

	gst[1] = make([]int, 60)
	gst[1][10] = 6
	gst[1][1] = 1

	gid, min := mostMinutesAsleep(gst)
	if gid != 0 {
		t.Error("expected guard 0 to have slept the longest")
	}

	if min != 10 {
		t.Error("expected minute 10 to have been the longest")
	}
}
