package pomodoro

import (
	"context"
	"fmt"
	"time"

	"github.com/charmbracelet/bubbles/progress"
	tea "github.com/charmbracelet/bubbletea"
	"michaelfcollins3.dev/projects/time/internal/dbid"
)

type model struct {
	ctx              context.Context
	startTime        time.Time
	minutesRemaining int
	secondsRemaining int
	progress         progress.Model
	err              error
	pomodoroID       dbid.ID
}

func newModel(ctx context.Context, startTime time.Time) model {
	return model{
		ctx:              ctx,
		startTime:        startTime,
		minutesRemaining: 25,
		secondsRemaining: 0,
		progress: progress.New(
			progress.WithSolidFill("#00ff00"),
			progress.WithWidth(50),
			progress.WithoutPercentage(),
		),
	}
}

func (m model) Init() tea.Cmd {
	return tea.Batch(
		createPomodoro(m.ctx, m.startTime),
		tick(),
	)
}

func (m model) Update(msg tea.Msg) (tea.Model, tea.Cmd) {
	switch msg := msg.(type) {
	case tickMsg:
		m = m.updateTime(msg)
		percent := 1 - (float64(m.minutesRemaining*60+m.secondsRemaining) /
			float64(pomodoroDuration.Seconds()))
		if percent >= 0.8 {
			m.progress.FullColor = "#ffff00"
		}

		if percent >= 0.95 {
			m.progress.FullColor = "#ff0000"
		}

		return m, tea.Batch(
			m.progress.SetPercent(percent),
			tick(),
		)

	case tea.KeyMsg:
		switch msg.String() {
		case "ctrl+c", "q":
			return m, tea.Quit
		}

	case progress.FrameMsg:
		progressModel, cmd := m.progress.Update(msg)
		m.progress = progressModel.(progress.Model)
		return m, cmd

	case errorMsg:
		m.err = msg.err
		return m, tea.Quit

	case pomodoroCreatedMsg:
		m.pomodoroID = msg.ID
		return m, nil
	}

	return m, nil
}

func (m model) View() string {
	return fmt.Sprintf(
		"%s %d:%02d",
		m.progress.View(),
		m.minutesRemaining,
		m.secondsRemaining,
	)
}

func (m model) updateTime(msg tickMsg) model {
	timeRemaining := pomodoroDuration - msg.time.Sub(m.startTime)
	expired := timeRemaining <= 0
	if expired {
		m.minutesRemaining = 0
		m.secondsRemaining = 0
	} else {
		m.minutesRemaining = int(timeRemaining.Minutes())
		m.secondsRemaining = int(timeRemaining.Seconds()) % 60
	}

	return m
}
