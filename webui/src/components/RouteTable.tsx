import React from 'react';
import {
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Button,
  Box,
  Typography,
  CircularProgress, // Add CircularProgress import if you want it here
} from '@mui/material';
import { Edit as EditIcon, Delete as DeleteIcon } from '@mui/icons-material';
// Assuming 'Route' type is defined in '../api/routes' as discussed,
// or in a centralized 'types.ts' as you've imported.
import type { Route } from '../types'; // Adjust path if your types are elsewhere

// Propiedades que espera el componente RouteTable
interface RouteTableProps {
  routes: Route[]; // Array de rutas a mostrar
  onEdit: (route: Route) => void; // Función para manejar la edición de una ruta
  onDelete: (id: number) => void; // Función para manejar la eliminación de una ruta (ID es string)
  loading: boolean; // Estado de carga
  error: string | null; // Mensaje de error
}

const RouteTable: React.FC<RouteTableProps> = ({ routes, onEdit, onDelete, loading, error }) => {
  // If loading, display a spinner and message.
  if (loading) {
    return (
      <Box sx={{ display: 'flex', flexDirection: 'column', alignItems: 'center', mt: 4 }}>
        <CircularProgress sx={{ mb: 2 }} />
        <Typography variant="h6">Loading routes...</Typography>
      </Box>
    );
  }

  // If there's an error, display the error message.
  if (error) {
    return (
      <Box sx={{ mt: 4, textAlign: 'center' }}>
        <Typography color="error" variant="h6">
          Error: {error}
        </Typography>
      </Box>
    );
  }

  // If no routes are found and not loading/error, display the "No routes" message.
  if (routes.length === 0 || !routes.map) {
    return (
      <Box sx={{ mt: 4, textAlign: 'center' }}>
        <Typography variant="h6">No routes found.</Typography>
        <Typography variant="body1">Click "Add New Route" to create one.</Typography>
      </Box>
    );
  }

  // Otherwise, render the table.
  return (
    <TableContainer component={Paper} sx={{ mt: 4 }}>
      <Table sx={{ minWidth: 650 }} aria-label="routes table">
        <TableHead>
          <TableRow>
            <TableCell>ID</TableCell>
            <TableCell>Path Pattern</TableCell>
            <TableCell>Target URL</TableCell>
            <TableCell>Method(s)</TableCell>
            <TableCell>Required Roles</TableCell>
            <TableCell>Required Permission</TableCell>
            <TableCell align="right">Actions</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {routes.map((route) => (
            <TableRow key={route.id}>
              <TableCell>{route.id}</TableCell>
              <TableCell>{route.path_pattern}</TableCell>
              <TableCell>{route.target_url}</TableCell>
              {/* Join array elements with a comma for display, or show '-' if empty */}
              <TableCell>{route.http_method || '-'}</TableCell>
              <TableCell>{JSON.stringify(route.required_roles)}</TableCell>
              <TableCell>{JSON.stringify(route.require_permission)}</TableCell>
              <TableCell align="right">
                <Button size="small" onClick={() => onEdit(route)}>
                  <EditIcon fontSize="small" />
                </Button>
                <Button size="small" color="error" onClick={() => onDelete(route.id)}>
                  <DeleteIcon fontSize="small" />
                </Button>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
};

export default RouteTable;