export interface Route {
    id: number; 
    path_pattern: string;
    target_url: string;
    http_method: string;
    required_roles: string[] | null;
    require_permission: string[] | null;
}

// Interfaz para los datos que se envían al crear una nueva ruta (sin ID)
export type CreateRouteData = Omit<Route, 'id'>;

// Interfaz para los datos que se envían al actualizar una ruta (parciales)
export type UpdateRouteData = Partial<Route>;


