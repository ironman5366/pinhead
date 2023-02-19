package models

import (
	"github.com/jackc/pgx/pgtype"
	"gorm.io/gorm"
)

type Document struct {
	gorm.Model
	Name     string
	Content  pgtype.JSONB
	Versions []DocumentVersion
}
