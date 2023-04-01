import React from 'react';
import {Container, Title} from "@mantine/core";
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import Home from "./pages/Home";
import CreateDocument from "./pages/CreateDocument";
import MainLayout from "./containers/MainLayout";

const router = createBrowserRouter([
  {
    path: "/",
    element:
        <MainLayout>
          <Home />
        </MainLayout>
  },
  {
    path: "/documents/create",
    element: <CreateDocument />
  }
])

function App() {
  return <Container fluid>
      <RouterProvider router={router} />
  </Container>
}

export default App;
