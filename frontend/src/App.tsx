import React from 'react';
import {Container} from "@mantine/core";
import {Route, Routes} from "react-router-dom";
import MainLayout from "./containers/MainLayout";
import GuestLayout from "./containers/GuestLayout";
import Login from "./pages/Login";
import Register from "./pages/Register";
import Documents from "./pages/Documents";
import Settings from "./pages/Settings";
import ContentTypes from "./pages/ContentTypes";
import CreateContentField from "./pages/CreateContentField";

function App() {
    return <Container fluid>
        <Routes>
            <Route path="/" element={<MainLayout />}>
                <Route path="/" element={<Documents />} />
                <Route path="/content_types/" element={<ContentTypes />} />
                <Route path="/content_fields/">
                    <Route path="new" element={<CreateContentField />} />
                </Route>
                <Route path="/settings/" element={<Settings />} />
            </Route>
            <Route path="/register" element={<GuestLayout>
                <Register />
            </GuestLayout>} />
            <Route path="/login" element={<GuestLayout>
                <Login />
            </GuestLayout>} />
        </Routes>
    </Container>
}

export default App;
