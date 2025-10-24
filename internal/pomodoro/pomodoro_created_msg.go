package pomodoro

import (
	"context"
	"fmt"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"gorm.io/gorm"
	appcontext "michaelfcollins3.dev/projects/time/internal/context"
	"michaelfcollins3.dev/projects/time/internal/database"
	"michaelfcollins3.dev/projects/time/internal/dbid"
)

type pomodoroCreatedMsg struct {
	ID dbid.ID
}

func createPomodoro(ctx context.Context, startTime time.Time) tea.Cmd {
	return func() tea.Msg {
		db := ctx.Value(appcontext.DBContextKey).(*gorm.DB)
		id := dbid.NewID()
		pomodoro := database.Pomodoro{
			Model: database.Model{
				ID: id,
			},
			StartTime: startTime,
		}

		timeoutCtx, cancel := context.WithTimeout(ctx, 5*time.Second)
		err := gorm.G[database.Pomodoro](db).Create(timeoutCtx, &pomodoro)
		cancel()
		if err != nil {
			return errorMsg{
				err: fmt.Errorf("failed to create pomodoro: %w", err),
			}
		}

		return pomodoroCreatedMsg{
			ID: id,
		}
	}
}
