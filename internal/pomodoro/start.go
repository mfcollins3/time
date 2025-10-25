package pomodoro

import (
	"context"
	_ "embed"
	"errors"
	"fmt"
	"os"
	"time"

	tea "github.com/charmbracelet/bubbletea"
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
