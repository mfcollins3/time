package database

import (
	"gorm.io/gorm"
	"michaelfcollins3.dev/projects/time/internal/dbid"
)

type Model struct {
	gorm.Model

	ID dbid.ID `gorm:"type:text,primaryKey"`
}
