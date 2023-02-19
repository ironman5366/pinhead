package models

import "gorm.io/gorm"

type DocumentVersion struct {
	gorm.Model
	DocumentID uint
}
