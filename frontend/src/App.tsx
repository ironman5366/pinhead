import React from 'react';
import {Container, Title} from "@mantine/core";
import {createBrowserRouter, RouterProvider} from "react-router-dom";
import Home from "./pages/Home";
import CreateDocument from "./pages/CreateDocument";
import MainLayout from "./containers/MainLayout";
import GuestLayout from "./containers/GuestLayout";
import Login from "./pages/Login";
import Register from "./pages/Register";

const router = createBrowserRouter([
    {
        path: "/",
        element:
            <MainLayout>
                <Home/>
            </MainLayout>
    },
    {
        path: "/login",
        element:
            <GuestLayout>
                <Login/>
            </GuestLayout>
    },
    {
        path: "/register",
        element: <GuestLayout>
            <Register/>
        </GuestLayout>
    },
    {
        path: "/documents/create",
        element: <CreateDocument/>
    }
])

function App() {
    return <Container fluid>
        <RouterProvider router={router}/>
    </Container>
}

export default App;
