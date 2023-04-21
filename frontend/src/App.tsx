import React from 'react';
import {Container} from "@mantine/core";
import {BrowserRouter, Route, Routes} from "react-router-dom";
import MainLayout from "./containers/MainLayout";
import GuestLayout from "./containers/GuestLayout";
import Login from "./pages/Login";
import Register from "./pages/Register";
import Documents from "./pages/Documents";
import Settings from "./pages/Settings";
import ContentTypes from "./pages/ContentTypes";

function App() {
    return <Container fluid>
        <BrowserRouter>
            <Routes>
                <Route path="/" element={<MainLayout />}>
                    <Route path="/" element={<Documents />} />
                    <Route path="/content_types/" element={<ContentTypes />} />
                    <Route path="/settings/" element={<Settings />} />
                </Route>
                <Route path="/register" element={<GuestLayout>
                    <Register />
                </GuestLayout>} />
                <Route path="/login" element={<GuestLayout>
                    <Login />
                </GuestLayout>} />
            </Routes>
        </BrowserRouter>
    </Container>
}

export default App;
