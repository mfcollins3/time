//go:build docker

package pomodoro

func playAlarmSound() (chan bool, error) {
	done := make(chan bool)
	done <- true
	return done, nil
}
