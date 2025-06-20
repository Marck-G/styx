// src/pages/RoutesManagement.tsx
import React, { useEffect, useState } from 'react';
import {
  Container, Typography, Box, Button, CircularProgress, Alert, // Quita Table/TableContainer/etc. de aquí
} from '@mui/material';
import { Add as AddIcon } from '@mui/icons-material';
import {
  fetchRoutes,
  createRoute,
  updateRoute,
  deleteRoute,
} from '../api/routes';
import RouteForm from '../components/RouteForm'; // Importa el formulario
import RouteTable from '../components/RouteTable'; // Importa la tabla
import type { CreateRouteData, Route, RouteFormData, UpdateRouteData } from '../types';

const RoutesManagement: React.FC = () => {
  const [routes, setRoutes] = useState<Route[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [openForm, setOpenForm] = useState(false);
  const [editingRoute, setEditingRoute] = useState<Route | null>(null);

  // Función para cargar las rutas desde la API
  const loadRoutes = async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await fetchRoutes();
      setRoutes(data);
    } catch (err) {
      setError('Failed to load routes. Please check your API connection.');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  // Cargar las rutas al montar el componente (una vez)
  useEffect(() => {
    loadRoutes();
  }, []);

  // Manejadores para abrir el formulario
  const handleOpenCreate = () => {
    setEditingRoute(null); // No hay ruta para editar, es una creación nueva
    setOpenForm(true);
  };

  const handleOpenEdit = (route: Route) => {
    setEditingRoute(route); // Establece la ruta a editar
    setOpenForm(true);
  };

  // Manejador para eliminar una ruta
  const handleDelete = async (id: number) => {
    if (window.confirm('Are you sure you want to delete this route?')) {
      try {
        await deleteRoute(id);
        await loadRoutes(); // Recarga las rutas después de la eliminación exitosa
        alert('Route deleted successfully!'); // Feedback al usuario
      } catch (err) {
        setError('Failed to delete route.');
        console.error(err);
      }
    }
  };



  // Manejador para guardar (crear o actualizar) una ruta
  const handleSaveRoute = async (routeData: RouteFormData ) => {
    console.debug("OBJ:", routeData)
    try {
      if (editingRoute) {
        // Modo edición
        await updateRoute(editingRoute.id, routeData as UpdateRouteData);
        alert('Route updated successfully!');
      } else {
        // Modo creación
          await createRoute(routeData as CreateRouteData);
          
        alert('Route created successfully!');
      }
      setOpenForm(false); // Cierra el formulario
      setEditingRoute(null); // Limpia la ruta en edición
      await loadRoutes(); // Recarga las rutas para ver los cambios
    } catch (err) {
      setError('Failed to save route. Please check your input.');
      console.error(err);
    }
  };

  return (
    <Container maxWidth="lg" sx={{ p: 3}}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h4" component="h1">
          Routes
        </Typography>
      
        <Button variant="contained" startIcon={<AddIcon />} onClick={handleOpenCreate}>
          Add New Route
        </Button>
      </Box>

      {/* Indicadores de carga y error a nivel de página */}
      {loading && (
        <Box sx={{ display: 'flex', justifyContent: 'center', mt: 4 }}>
          <CircularProgress />
        </Box>
      )}
      {error && <Alert severity="error" sx={{ mt: 4 }}>{error}</Alert>}

      {/* Renderiza la tabla de rutas solo si no está cargando y no hay error */}
      {!loading && !error && (
        <RouteTable
          routes={routes}
          onEdit={handleOpenEdit}
          onDelete={handleDelete}
          loading={loading} // Se pasa para que RouteTable maneje su propia lógica de carga/error (opcional, pero útil)
          error={error}
        />
      )}

      {/* El formulario de ruta */}
      <RouteForm
        open={openForm}
        onClose={() => { setOpenForm(false); setEditingRoute(null); }}
        onSave={handleSaveRoute}
        initialData={editingRoute}
      />
    </Container>
  );
};

export default RoutesManagement;