package dbid

import (
	"database/sql/driver"
	"fmt"

	"github.com/google/uuid"
)

type ID uuid.UUID

func NewID() ID {
	id, err := uuid.NewV7()
	if err != nil {
		panic(fmt.Errorf("failed to generate UUIDv7: %w", err))
	}

	return ID(id)
}

func (id *ID) Scan(src interface{}) error {
	u := uuid.UUID{}
	err := u.Scan(src)
	if err != nil {
		return err
	}

	*id = ID(u)
	return nil
}

func (id ID) Value() (driver.Value, error) {
	return uuid.UUID(id).Value()
}
