//go:build !docker

package pomodoro

import (
	"bytes"
	"io"
	"os"
	"time"

	"github.com/gopxl/beep"
	"github.com/gopxl/beep/mp3"
	"github.com/gopxl/beep/speaker"
)

func playAlarmSound() (chan bool, error) {
	done := make(chan bool)
	if os.Getenv("TIME_NO_SOUND") == "1" {
		done <- true
		return done, nil
	}

	streamer, format, err := mp3.Decode(
		io.NopCloser(bytes.NewReader(alarmSound)),
	)
	if err != nil {
		return done, err
	}

	defer func() {
		_ = streamer.Close()
	}()

	err = speaker.Init(
		format.SampleRate,
		format.SampleRate.N(time.Second/10),
	)
	if err != nil {
		return done, err
	}

	speaker.Play(beep.Seq(streamer, beep.Callback(func() {
		done <- true
	})))

	return done, nil
}
