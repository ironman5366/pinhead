import React from "react";
import {Card, Title} from "@mantine/core";
import PinheadDocument from "../../models/pinheadDocument";

export interface DocumentCardProps {
    document: PinheadDocument;
}

export default function DocumentCard({document}: DocumentCardProps) {
    return <Card>
        <Card.Section>
            <Title order={3}>
                {document.title}
            </Title>
        </Card.Section>
    </Card>
}
