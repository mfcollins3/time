package pomodoro

import (
	"bytes"
	"context"
	_ "embed"
	"errors"
	"fmt"
	"io"
	"os"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/gen2brain/beeep"
	"github.com/gopxl/beep"
	"github.com/gopxl/beep/mp3"
	"github.com/gopxl/beep/speaker"
	"gorm.io/gorm"
	appcontext "michaelfcollins3.dev/projects/time/internal/context"
	"michaelfcollins3.dev/projects/time/internal/database"
)

//go:embed alarm.mp3
var alarmSound []byte

func Start(ctx context.Context) error {
	startTime := time.Now()
	pomodoroCtx, cancel := context.WithTimeout(ctx, pomodoroDuration)
	p := tea.NewProgram(
		newModel(ctx, startTime),
		tea.WithContext(pomodoroCtx),
	)
	m, err := p.Run()
	cancel()
	completed := errors.Is(err, context.DeadlineExceeded)
	if err != nil && !completed {
		return err
	}

	model := m.(model)
	if model.err != nil {
		return model.err
	}

	if completed {
		timeoutCtx, timeoutCancel := context.WithTimeout(ctx, 10*time.Second)
		defer timeoutCancel()

		done, err := playAlarmSound()
		if err != nil {
			fmt.Fprintf(os.Stderr, "Failed to play alarm sound: %v\n", err)
		}

		err = showDesktopNotification()
		if err != nil {
			fmt.Fprintf(
				os.Stderr,
				"Failed to show desktop notification: %v\n",
				err,
			)
		}

		fmt.Println("Pomodoro completed! Take a break!")

		err = completePomodoro(ctx, model)
		if err != nil {
			return err
		}

		select {
		case <-done:
		case <-timeoutCtx.Done():
		}
	}

	return nil
}

func playAlarmSound() (chan bool, error) {
	done := make(chan bool)
	if os.Getenv("TIME_NO_SOUND") == "1" {
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

func showDesktopNotification() error {
	if os.Getenv("TIME_NO_DESKTOP_NOTIFICATION") == "1" {
		return nil
	}

	err := beeep.Notify(
		"Pomodoro Complete",
		"Congratulations, your pomodoro is complete! You should now take a break before starting another pomodoro.",
		"",
	)
	if err != nil {
		return fmt.Errorf("failed to send desktop notification: %w", err)
	}

	return nil
}

func completePomodoro(ctx context.Context, model model) error {
	db := ctx.Value(appcontext.DBContextKey).(*gorm.DB)
	dbCtx, dbCancel := context.WithTimeout(ctx, 5*time.Second)
	rows, err := gorm.G[database.Pomodoro](db).
		Where("id = ?", model.pomodoroID).
		Update(dbCtx, "end_time", model.startTime.Add(pomodoroDuration))
	dbCancel()
	if err != nil {
		return fmt.Errorf("failed to update pomodoro end time: %w", err)
	}

	if rows == 0 {
		return fmt.Errorf(
			"failed to update pomodoro end time: no rows affected",
		)
	}

	return nil
}
