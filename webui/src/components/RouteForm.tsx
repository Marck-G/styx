import { useEffect, useState } from "react";
import type { RouteFormData, RouteFormProps } from "../types";
import CloseIcon from '@mui/icons-material/Close';
import AddIcon from '@mui/icons-material/Add';
import { Box, Button, Chip, Dialog, DialogActions, DialogContent, DialogTitle, FormControl, IconButton, InputLabel, MenuItem, OutlinedInput, Select, TextField } from "@mui/material";



// Listas de opciones predefinidas para los selectores
const ALL_METHODS = [
  "GET",
  "POST",
  "PUT",
  "DELETE",
  "PATCH",
  "OPTIONS",
  "HEAD",
  "ALL"
];

const RouteForm: React.FC<RouteFormProps> = ({
  open,
  onClose,
  onSave,
  initialData,
}) => {
  const [formData, setFormData] = useState<RouteFormData>({
    path_pattern: "",
    target_url: "",
    http_method: "", // Inicialmente vacío o un array
    required_roles: [],
    require_permission: [],
  });

  // Estado temporal para el nuevo rol/permiso que el usuario está escribiendo
  const [newRole, setNewRole] = useState<string>("");
  const [newPermission, setNewPermission] = useState<string>("");
  const [edit, setEdit] = useState<boolean>(false);
  useEffect(() => {
    if (initialData) {
      setEdit(true);
      setFormData({
        path_pattern: initialData.path_pattern,
        target_url: initialData.target_url,
        http_method: initialData.http_method ? initialData.http_method : "", // Si initialData.method es singular, lo convierte a array
        required_roles: initialData.required_roles ?? new Array<string>(),
        require_permission: initialData.require_permission ?? new Array<string>()
      });
    } else {
      setEdit(false);
      setFormData({
        path_pattern: "",
        target_url: "",
        http_method: "",
        required_roles: [],
        require_permission: [],
      });
    }
    // Resetea los campos de entrada dinámicos al abrir/cerrar el formulario
    setNewRole("");
    setNewPermission("");
  }, [initialData, open]);

   const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData((prev) => ({ ...prev, [name]: value }));
  };

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const handleMethodChange = (e: any) => { // Específico para el Select de método
    const { value } = e.target;
    setFormData((prev) => ({
      ...prev,
      http_method: value, // Permite selección múltiple
    }));
  };

  // --- Lógica para Roles Dinámicos ---
  const handleAddRole = () => {
    if (newRole.trim() && !formData.required_roles.includes(newRole.trim())) {
      setFormData((prev) => ({
        ...prev,
        required_roles: [...prev.required_roles, newRole.trim()],
      }));
      setNewRole(''); // Limpia el input
    }
  };

  const handleDeleteRole = (roleToDelete: string) => () => {
    setFormData((prev) => ({
      ...prev,
      required_roles: prev.required_roles.filter((role) => role !== roleToDelete),
    }));
  };

  // --- Lógica para Permisos Dinámicos ---
  const handleAddPermission = () => {
    if (newPermission.trim() && !formData.require_permission.includes(newPermission.trim())) {
      setFormData((prev) => ({
        ...prev,
        require_permission: [...prev.require_permission, newPermission.trim()],
      }));
      setNewPermission(''); // Limpia el input
    }
  };

  const handleDeletePermission = (permissionToDelete: string) => () => {
    setFormData((prev) => ({
      ...prev,
      require_permission: prev.require_permission.filter((perm) => perm !== permissionToDelete),
    }));
  };

  const handleSubmit = () => {
    // Aquí puedes añadir validaciones adicionales si es necesario
    // Por ejemplo, si method debe ser un solo elemento para tu API:
    const dataToSend: RouteFormData = {
      ...formData,
      // Si la API solo acepta un método, toma el primero o null
      http_method: formData.http_method ?? (initialData?.http_method ?? "") , // <-- AJUSTAR SEGÚN LO QUE TU API ESPERA
      // Si la API solo acepta un permiso (string o null) y tienes un array, toma el primero o null
      require_permission: formData.require_permission.length > 0 ? formData.require_permission : new Array<string>(), // <-- AJUSTAR
      required_roles: formData.required_roles.length > 0 ? formData.required_roles : new Array<string>(), // <-- AJUSTAR
    };
    console.debug("On Save data:", dataToSend)
    onSave(dataToSend );
  };
  return (
    <Dialog open={open} onClose={onClose} fullWidth maxWidth="sm">
      <DialogTitle>{initialData ? 'Edit Route' : 'Create New Route'}</DialogTitle>
      <DialogContent>
        <TextField
          autoFocus
          margin="dense"
          name="path_pattern"
          label="Path Pattern (e.g., /api/v1/users/*)"
          type="text"
          fullWidth
          variant="outlined"
          value={formData.path_pattern}
          onChange={handleChange}
          sx={{ mb: 2 }}
        />
        <TextField
          margin="dense"
          name="target_url"
          label="Target URL (e.g., http://localhost:8080/v1/)"
          type="url"
          fullWidth
          variant="outlined"
          value={formData.target_url}
          onChange={handleChange}
          sx={{ mb: 2 }}
        />
        {/* Selector de Método HTTP - Ahora permite selección múltiple */}
        <FormControl fullWidth margin="dense" sx={{ mb: 2 }}>
          <InputLabel>Method</InputLabel>
          <Select
            name="method"
            value={formData.http_method}
            onChange={handleMethodChange}
            readOnly= {edit}
            input={<OutlinedInput label="Method" />}
            renderValue={(selected) => {
              return (
              <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5 }}>
                  <Chip key={selected} label={selected} />
              </Box>
            )}}
          >
            {ALL_METHODS.map((method) => (
              <MenuItem key={method} value={method}>
                {method}
              </MenuItem>
            ))}
          </Select>
        </FormControl>

        {/* Input dinámico para Required Roles */}
        <Box sx={{ mb: 2 }}>
          <InputLabel shrink htmlFor="roles-input">Required Roles</InputLabel>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mt: 1 }}>
            <TextField
              id="roles-input"
              size="small"
              variant="outlined"
              placeholder="Add role"
              value={newRole}
              onChange={(e) => setNewRole(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleAddRole()} // Permite agregar con Enter
              fullWidth
            />
            <IconButton color="primary" onClick={handleAddRole}>
              <AddIcon  />
            </IconButton>
          </Box>
          <Box sx={{ mt: 1, display: 'flex', flexWrap: 'wrap', gap: 1 }}>
            {formData.required_roles.map((role) => (
              <Chip
                key={role}
                label={role}
                onDelete={handleDeleteRole(role)}
                deleteIcon={<CloseIcon />}
              />
            ))}
          </Box>
        </Box>

        {/* Input dinámico para Required Permission */}
        <Box sx={{ mb: 2 }}>
          <InputLabel shrink htmlFor="permission-input">Required Permission</InputLabel>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mt: 1 }}>
            <TextField
              id="permission-input"
              size="small"
              variant="outlined"
              placeholder="Add permission"
              value={newPermission}
              onChange={(e) => setNewPermission(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleAddPermission()} // Permite agregar con Enter
              fullWidth
            />
            <IconButton color="primary" onClick={handleAddPermission}>
              <AddIcon />
            </IconButton>
          </Box>
          <Box sx={{ mt: 1, display: 'flex', flexWrap: 'wrap', gap: 1 }}>
            {formData.require_permission.map((permission) => (
              <Chip
                key={permission}
                label={permission}
                onDelete={handleDeletePermission(permission)}
                deleteIcon={<CloseIcon />}
              />
            ))}
          </Box>
        </Box>
      </DialogContent>
      <DialogActions>
        <Button onClick={onClose}>Cancel</Button>
        <Button onClick={handleSubmit} variant="contained" color="primary">
          {initialData ? 'Save Changes' : 'Create Route'}
        </Button>
      </DialogActions>
    </Dialog>
  );
};

export default RouteForm;