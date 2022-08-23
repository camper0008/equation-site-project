package db_impl

import (
	"equation-site-backend/models"
	"fmt"

	"golang.org/x/crypto/bcrypt"
)

type MemoryDb struct {
	users     []models.User
	equations []models.Equation
	sessions  map[models.Session]models.User
}

func (db *MemoryDb) Init() {
	if pass, err := bcrypt.GenerateFromPassword([]byte("pass"), bcrypt.DefaultCost); err == nil {
		db.users = []models.User{
			models.User{
				Username:   models.Username("user"),
				Password:   models.Password(pass),
				Permission: models.ContributorPermission,
			},
		}
		db.sessions = make(map[models.Session]models.User)
	}
}

func (db *MemoryDb) UserFromUsername(username models.Username) (models.User, error) {
	for _, user := range db.users {
		if user.Username == username {
			return user, nil
		}
	}
	return models.User{}, fmt.Errorf("user does not exist")
}

func (db *MemoryDb) UserFromSession(session models.Session) (models.User, error) {
	for userSession, user := range db.sessions {
		if userSession == session {
			return user, nil
		}
	}
	return models.User{}, fmt.Errorf("session does not exist")
}

func (db *MemoryDb) BindSessionToUser(session models.Session, user models.User) error {
	if _, found := db.sessions[session]; found != false {
		return fmt.Errorf("session collision")
	}
	db.sessions[session] = user
	return nil
}

func (db *MemoryDb) UnbindSession(session models.Session) error {
	if _, found := db.sessions[session]; found == false {
		return fmt.Errorf("session not bound")
	}
	delete(db.sessions, session)
	return nil
}

func (db *MemoryDb) InsertUser(user models.User) error {
	if _, err := db.UserFromUsername(user.Username); err == nil {
		return fmt.Errorf("username taken")
	}
	db.users = append(db.users, user)
	return nil
}

func (db *MemoryDb) InsertEquation(eq models.Equation) error {
	db.equations = append(db.equations, eq)
	return nil
}

func (db *MemoryDb) EquationFromId(id models.Id) (models.Equation, error) {
	for _, eq := range db.equations {
		if eq.Id == id {
			return eq, nil
		}
	}
	return models.Equation{}, fmt.Errorf("equation does not exist")
}

func (db *MemoryDb) UpdateEquation(newEq models.Equation) error {
	for idx, eq := range db.equations {
		if eq.Id == newEq.Id {
			db.equations[idx] = newEq
			return nil
		}
	}
	return fmt.Errorf("equation does not exist")
}

func (db *MemoryDb) AllEquations() ([]models.PreviewableEquation, error) {
	res := make([]models.PreviewableEquation, len(db.equations))
	for idx, eq := range db.equations {
		res[idx] = models.PreviewableEquation{
			Id:          eq.Id,
			Title:       eq.Title,
			DateCreated: eq.DateCreated,
		}
	}
	return res, nil
}

func (db *MemoryDb) RemoveEquation(givenEq models.Equation) error {
	for idx, eq := range db.equations {
		if eq.Id == givenEq.Id {
			db.equations[idx], db.equations[len(db.equations)-1] = db.equations[len(db.equations)-1], db.equations[idx]
			db.equations = db.equations[:len(db.equations)-2]
			return nil
		}
	}
	return fmt.Errorf("equation does not exist")
}
