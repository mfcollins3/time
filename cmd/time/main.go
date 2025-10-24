package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"path"

	"gorm.io/gorm"
	appcontext "michaelfcollins3.dev/projects/time/internal/context"
	"michaelfcollins3.dev/projects/time/internal/database"
	"michaelfcollins3.dev/projects/time/internal/pomodoro"
)

func main() {
	db, err := createDatabase()
	if err != nil {
		log.Fatalf("Failed to create database: %v", err)
	}

	ctx := context.WithValue(context.Background(), appcontext.DBContextKey, db)
	if err := pomodoro.Start(ctx); err != nil {
		log.Fatalf("An error occurred while running the command: %v", err)
	}
}

func createDatabase() (*gorm.DB, error) {
	homePath, err := os.UserHomeDir()
	if err != nil {
		return nil, fmt.Errorf("failed to get user home directory: %w", err)
	}

	dbDir := path.Join(homePath, ".mfcollins3/time")
	dbPath := path.Join(dbDir, "time.db")
	if err := os.MkdirAll(dbDir, 0755); err != nil {
		return nil, fmt.Errorf(
			"failed to create the ~/.mfcollins3/time directory: %w",
			err,
		)
	}

	db, err := database.NewDB(dbPath)
	if err != nil {
		return nil, err
	}

	return db, nil
}
