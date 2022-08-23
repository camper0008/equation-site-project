package models

type Id string
type Session string
type UserId Id
type Username string
type Password string
type DateCreated string

type DB interface {
	InsertUser(User) error
	UserFromUsername(Username) (User, error)
	UserFromSession(Session) (User, error)
	BindSessionToUser(Session, User) error
	UnbindSession(Session) error
	InsertEquation(Equation) error
	EquationFromId(Id) (Equation, error)
	UpdateEquation(Equation) error
	RemoveEquation(Equation) error
	AllEquations() ([]PreviewableEquation, error)
}

type User struct {
	Id          UserId
	Username    Username
	Password    Password
	Permission  Permission
	DateCreated DateCreated
}

type Equation struct {
	Id          Id
	Title       string
	Content     string
	Creator     User
	DateCreated DateCreated
}

type PreviewableEquation struct {
	Id          Id
	Title       string
	DateCreated DateCreated
}

type Permission int

const (
	UndefinedPermission = iota
	UserPermission
	ContributorPermission
	RootPermission
)

func (p Permission) String() string {
	switch p {
	case UndefinedPermission:
		return "Undefined"
	case UserPermission:
		return "User"
	case ContributorPermission:
		return "Contributor"
	case RootPermission:
		return "Root"
	}

	return "Unknown"
}
