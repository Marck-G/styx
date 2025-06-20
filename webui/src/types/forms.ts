import type { Route } from "./route";

// Definición de tipos para los datos del formulario (similar a CreateRouteData)
export interface RouteFormData {
  path_pattern: string;
  target_url: string;
  http_method: string ;
  required_roles: string[];
  require_permission: Array<string> ;
}

// Propiedades que espera el componente RouteForm
export interface RouteFormProps {
  open: boolean; // Controla si el diálogo está visible
  onClose: () => void; // Función para cerrar el diálogo
  onSave: (data: RouteFormData) => void; // Función que se llama al guardar el formulario
  initialData?: Route | null; // Datos iniciales para precargar el formulario (para edición)
}