// src/App.tsx
import { ThemeProvider, createTheme, CssBaseline } from '@mui/material';
import { BrowserRouter, Routes, Route } from 'react-router-dom'; // Importa componentes de React Router

import DashboardLayout from './layouts/DashboardLayout'; // Importa tu layout
import RoutesManagement from './pages/RoutesManagement'; // Tu página principal de rutas
import AboutPage from './pages/AboutPage'; // Tu nueva página About

// Define un tema básico de Material-UI
const theme = createTheme({
  palette: {
    mode: 'light', // Puedes cambiar a 'dark' o personalizarlo a tu gusto
    primary: {
      main: '#2C5559', // Un azul estándar de MUI
    },
    secondary: {
      main: '#F4F2EE', // Un rosa estándar de MUI
    },
  },
});

function App() {
  return (
    <ThemeProvider theme={theme}>
      <CssBaseline /> {/* Resetea CSS base y aplica estilos de Material Design */}
      <BrowserRouter> {/* Envuelve toda la aplicación para habilitar el enrutamiento */}
        <Routes> {/* Define tus grupos de rutas */}
          <Route path="/" element={<DashboardLayout />}> {/* La ruta padre que usa el layout */}
            {/* Rutas anidadas que se renderizarán dentro del <Outlet> del DashboardLayout */}
            <Route index element={<RoutesManagement />} /> {/* Ruta por defecto para "/" */}
            <Route path="about" element={<AboutPage />} /> {/* Ruta para "/about" */}
            {/* Puedes añadir más rutas aquí si tienes más páginas admin */}
            {/* <Route path="users" element={<UsersManagementPage />} /> */}
          </Route>
          {/* Aquí podrías tener rutas sin layout (ej. una página de login si fuera necesario) */}
          {/* <Route path="/login" element={<LoginPage />} /> */}
        </Routes>
      </BrowserRouter>
    </ThemeProvider>
  );
}

export default App;