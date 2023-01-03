// trait object
// It has two pointers
// 1. one pointer goes to the data(the instance of the struct)
// 2. another pointer goes to a map of method call names to function pointers(virtual method table)

// trade off
// 1. have additional runtime cose
// 2. produce smaller code as the method won't be duplicated for each concrete type
