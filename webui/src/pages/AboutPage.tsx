// src/pages/AboutPage.tsx
import React from 'react';
import { Box, Typography, Container } from '@mui/material';

const AboutPage: React.FC = () => {
  return (
    <Container maxWidth="md" sx={{ mt: 4, mb: 4 }}>
      <Box>
        <Typography variant="h4" component="h1" gutterBottom>
          About API Gateway Admin
        </Typography>
        <Typography variant="body1" paragraph>
          This is an administrative interface for managing routes in your Rust API Gateway.
          It allows you to view, create, edit, and delete API routes dynamically.
        </Typography>
        <Typography variant="body1" paragraph>
          Developed with React, Material-UI, and powered by an Axum (Rust) backend.
        </Typography>
        <Typography variant="body2" color="text.secondary">
          Version: 1.0.0
        </Typography>
      </Box>
    </Container>
  );
};

export default AboutPage;