package util

import (
	"fmt"
	"github.com/ironman5366/pinhead/backend/pkg/models"
	"gorm.io/driver/postgres"
	"gorm.io/gorm"
	"os"
)

var DB *gorm.DB

func InitializeDatabase() error {
	dsn := fmt.Sprintf("host=%s user=%s password=%s dbname=%s port=5432", os.Getenv("DB_HOST"),
		os.Getenv("DB_USER"), os.Getenv("DB_PASS"), os.Getenv("DB_NAME"))

	db, err := gorm.Open(postgres.Open(dsn), &gorm.Config{})
	if err != nil {
		return err
	}

	err = db.AutoMigrate(&models.Document{}, &models.DocumentVersion{})
	if err != nil {
		return err
	}

	return nil
}

