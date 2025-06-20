import axios from "axios";
import type { CreateRouteData, Route, UpdateRouteData } from "../types";

const API_BASE = "/admin";

interface ApiRoute {
    id?: number; 
    path_pattern: string;
    target_url: string;
    http_method: string;
    required_roles: string | null;
    require_permission: string | null;
}

class Mapper {
  public static toDomain(infra: ApiRoute): Route {
    return {
      id: infra.id ?? -1,
      http_method: infra.http_method,
      path_pattern: infra.path_pattern,
      target_url: infra.target_url,
      require_permission: infra.require_permission ? JSON.parse(infra.require_permission) : null,
      required_roles: infra.required_roles ? JSON.parse(infra.required_roles): null
    }
  }

  public static toInfra(domain: Route) : ApiRoute{
    return {
      id: domain.id,
      http_method: domain.http_method,
      target_url: domain.target_url,
      path_pattern: domain.path_pattern,
      require_permission: JSON.stringify(domain.require_permission),
      required_roles: JSON.stringify(domain.required_roles)
    }
  }

  public static createInfra(domain: CreateRouteData) : ApiRoute{
    return {
      http_method: domain.http_method,
      target_url: domain.target_url,
      path_pattern: domain.path_pattern,
      require_permission: JSON.stringify(domain.require_permission),
      required_roles: JSON.stringify(domain.required_roles)
    }
  }
}

/**
 * Obtiene todas las rutas de la API.
 * @returns Una promesa que resuelve con un array de rutas.
 */
export const fetchRoutes = async (): Promise<Route[]> => {
  try {
    const response = await axios.get<ApiRoute[]>(`${API_BASE}/routes`);
    let data: ApiRoute[] =  response.data;
    let out: Route[] = data.map((route) =>  Mapper.toDomain(route))
    return out;
  } catch (error) {
    console.error('Error fetching routes:', error);
    throw error; // Propaga el error para que la UI pueda manejarlo
  }
};

/**
 * Crea una nueva ruta.
 * @param routeData Los datos de la nueva ruta (sin ID).
 * @returns Una promesa que resuelve con la ruta creada (incluyendo su ID).
 */
export const createRoute = async (routeData: CreateRouteData): Promise<Route> => {
  try {
    console.debug("To API: ", Mapper.createInfra(routeData));

    const response = await axios.post<ApiRoute>(`${API_BASE}/routes`, Mapper.createInfra(routeData));
    return Mapper.toDomain(response.data);
  } catch (error) {
    console.error('Error creating route:', error);
    throw error;
  }
};

/**
 * Actualiza una ruta existente.
 * @param id El ID de la ruta a actualizar.
 * @param routeData Los datos parciales de la ruta a actualizar.
 * @returns Una promesa que resuelve con la ruta actualizada.
 */
export const updateRoute = async (id: number, routeData: UpdateRouteData): Promise<Route> => {
  try {
    let request: any = {...routeData};
    if (routeData.require_permission) request.require_permission = JSON.stringify(routeData.require_permission);
    if (routeData.required_roles) request.required_roles = JSON.stringify(routeData.required_roles);
    // console.debug("To API: ", request);
    const response = await axios.put<ApiRoute>(`${API_BASE}/routes/${id}`, request);
    return Mapper.toDomain(response.data);
  } catch (error) {
    console.error(`Error updating route ${id}:`, error);
    throw error;
  }
};

/**
 * Elimina una ruta.
 * @param id El ID de la ruta a eliminar.
 * @returns Una promesa que resuelve cuando la ruta ha sido eliminada.
 */
export const deleteRoute = async (id: number): Promise<void> => {
  try {
    await axios.delete(`${API_BASE}/routes/${id}`);
  } catch (error) {
    console.error(`Error deleting route ${id}:`, error);
    throw error;
  }
};