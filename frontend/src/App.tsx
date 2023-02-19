import React from 'react';
import {Container, Title} from "@mantine/core";
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import Home from "./pages/Home";
import CreateDocument from "./pages/CreateDocument";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Home />
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
