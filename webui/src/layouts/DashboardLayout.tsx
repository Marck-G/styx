// src/layouts/DashboardLayout.tsx
import React, { useState } from 'react';
import {
  AppBar,
  Toolbar,
  Typography,
  IconButton,
  Drawer,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Box,
  CssBaseline,
} from '@mui/material';
import MenuIcon from '@mui/icons-material/Menu';
import HomeIcon from '@mui/icons-material/Home';
import InfoIcon from '@mui/icons-material/Info';
import { Link, Outlet } from 'react-router-dom'; // Importa Link y Outlet de react-router-dom

interface DashboardLayoutProps {
  children?: React.ReactNode; // Content that goes inside the layout (deprecated with Outlet, but good to keep)
}

const drawerWidth = 240; // Ancho del menú lateral

const DashboardLayout: React.FC<DashboardLayoutProps> = () => {
  const [mobileOpen, setMobileOpen] = useState(false);

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  // Contenido del menú lateral
  const drawer = (
    <Box sx={{ p: 2 }}> {/* Añade padding al Box del drawer */}
      <Toolbar /> {/* Espacio para el AppBar */}
      <Typography variant="h6" sx={{ my: 2, textAlign: 'center' }}>
      <Box sx={{display: "flex", alignContent: "center", justifyContent: "center"}}>

      <Box component="img" sx={{width: 100, }} src='/icon.png' />
      </Box>
        Admin Panel
      </Typography>
      <List>
        <ListItem disablePadding>
          <ListItemButton component={Link} to="/"> {/* Link a la raíz (Home/RoutesManagement) */}
            <ListItemIcon>
              <HomeIcon />
            </ListItemIcon>
            <ListItemText primary="Home" />
          </ListItemButton>
        </ListItem>
        <ListItem disablePadding>
          <ListItemButton component={Link} to="/about"> {/* Link a la página About */}
            <ListItemIcon>
              <InfoIcon />
            </ListItemIcon>
            <ListItemText primary="About" />
          </ListItemButton>
        </ListItem>
      </List>
    </Box>
  );

  return (
    <Box sx={{ display: 'flex' }}>
      <CssBaseline /> {/* Resetea CSS para Material Design */}

      {/* AppBar (Barra Superior) */}
      <AppBar
        position="fixed"
        sx={{
          width: { sm: `calc(100% - ${drawerWidth}px)` }, // Se ajusta si el drawer está fijo
          ml: { sm: `${drawerWidth}px` }, // Margen izquierdo para dejar espacio al drawer
        }}
      >
        <Toolbar>
          <IconButton
            color="inherit"
            aria-label="open drawer"
            edge="start"
            onClick={handleDrawerToggle}
            sx={{ mr: 2, display: { sm: 'none' } }} // Visible solo en pantallas pequeñas
          >
            <MenuIcon />
          </IconButton>
          <Typography variant="h6" noWrap component="div">
            Styx Admin
          </Typography>
        </Toolbar>
      </AppBar>

      {/* Drawer (Menú Lateral) */}
      <Box
        component="nav"
        sx={{ width: { sm: drawerWidth }, flexShrink: { sm: 0 } }}
        aria-label="mailbox folders"
      >
        {/* Drawer para pantallas pequeñas (oculto en escritorio) */}
        <Drawer
          variant="temporary"
          open={mobileOpen}
          onClose={handleDrawerToggle}
          ModalProps={{
            keepMounted: true, // Mejor rendimiento en móviles
          }}
          sx={{
            display: { xs: 'block', sm: 'none' },
            '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
          }}
        >
          {drawer}
        </Drawer>
        {/* Drawer para pantallas grandes (fijo en escritorio) */}
        <Drawer
          variant="permanent"
          sx={{
            display: { xs: 'none', sm: 'block' },
            '& .MuiDrawer-paper': { boxSizing: 'border-box', width: drawerWidth },
          }}
          open
        >
          {drawer}
        </Drawer>
      </Box>

      {/* Contenido Principal */}
      <Box
        component="main"
        sx={{
          flexGrow: 1,
          p: 3,
          width: { sm: `calc(100% - ${drawerWidth}px)` },
          mt: { xs: '56px', sm: '64px' }, // Ajusta el margen superior para el AppBar
          display: 'flex',            // Habilita Flexbox
          flexDirection: 'column',    // Organiza los elementos en columna (de arriba a abajo)
          alignItems: 'center',       // Centra horizontalmente los elementos (cuando flexDirection es 'column')
          justifyContent: 'center',
        }}
      >
        {/* Aquí se renderizarán los componentes de página (RoutesManagement, About, etc.) */}
        <Outlet />
      </Box>
    </Box>
  );
};

export default DashboardLayout;