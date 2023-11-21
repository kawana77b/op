package values

type valuesType interface {
	string | int | uint | float32
}

type values[T valuesType] struct {
	values []T
}

func New[T valuesType](s []T) *values[T] {
	return &values[T]{values: s}
}

func (s *values[T]) Get(idx uint) T {
	var res T
	if s.has(idx) {
		return s.values[idx]
	}

	return res
}

func (s *values[T]) has(idx uint) bool {
	return int(idx) < len(s.values)
}
