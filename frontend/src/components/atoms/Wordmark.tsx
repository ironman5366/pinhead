import React from 'react';
import {Image, Title} from "@mantine/core";
import {useTranslation} from "react-i18next";
import Logo from '../../assets/images/logo.png';

export default function Wordmark() {
    const { t } = useTranslation();
    return <div style={{
        display: "flex",
        alignItems: "center"
    }}>
        <Image src={Logo} width={128} style={{
            transform: "rotate(-45deg)"
        }} />
        <Title order={1} color="slate" style={{
            fontSize: 50,
            fontFamily: "Pacifico"
        }}>
            {t("title")}
        </Title>
    </div>
}
