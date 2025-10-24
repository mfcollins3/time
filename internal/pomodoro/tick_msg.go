package pomodoro

import (
	"time"

	tea "github.com/charmbracelet/bubbletea"
)

type tickMsg struct {
	time time.Time
}

func tick() tea.Cmd {
	return tea.Every(200*time.Millisecond, func(t time.Time) tea.Msg {
		return tickMsg{time: t}
	})
}
