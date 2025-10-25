//go:build !docker

package pomodoro

import (
	"fmt"
	"os"

	"github.com/gen2brain/beeep"
)

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
