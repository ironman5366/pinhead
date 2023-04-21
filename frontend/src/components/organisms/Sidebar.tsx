import React, {ReactElement, useState} from 'react';
import { createStyles, Navbar, Group, Code, getStylesRef, rem } from '@mantine/core';
import {
    IconSettings,
    IconSwitchHorizontal,
    IconLogout, IconFileDescription, IconTopologyRing, IconApi, TablerIconsProps,
} from '@tabler/icons-react';
import {useNavigate, Link } from "react-router-dom";

const useStyles = createStyles((theme) => ({
    header: {
        paddingBottom: theme.spacing.md,
        marginBottom: `calc(${theme.spacing.md} * 1.5)`,
        borderBottom: `${rem(1)} solid ${
            theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[2]
        }`,
    },

    footer: {
        paddingTop: theme.spacing.md,
        marginTop: theme.spacing.md,
        borderTop: `${rem(3)} solid #F3BAC0`,
    },

    link: {
        ...theme.fn.focusStyles(),
        display: 'flex',
        alignItems: 'center',
        textDecoration: 'none',
        fontSize: theme.fontSizes.lg,
        color: theme.colorScheme === 'dark' ? theme.colors.dark[1] : theme.colors.gray[7],
        padding: `${theme.spacing.xs} ${theme.spacing.sm}`,
        borderRadius: theme.radius.sm,
        fontWeight: 500,

        '&:hover': {
            backgroundColor: theme.colorScheme === 'dark' ? theme.colors.dark[6] : theme.colors.gray[0],
            color: theme.colorScheme === 'dark' ? theme.white : theme.black,

            [`& .${getStylesRef('icon')}`]: {
                color: theme.colorScheme === 'dark' ? theme.white : theme.black,
            },
        },
    },

    linkIcon: {
        ref: getStylesRef('icon'),
        color: theme.colorScheme === 'dark' ? theme.colors.dark[2] : theme.colors.gray[6],
        marginRight: theme.spacing.sm,
    },

    linkActive: {
        '&, &:hover': {
            backgroundColor: theme.colors.cerulean[9],
            color: 'white',
            [`& .${getStylesRef('icon')}`]: {
                color: "white"
            }
        }
    },
}));

interface AppTab {
    label: string
    icon: React.ComponentType<TablerIconsProps>
    link: string;
}

const tabs: AppTab[] = [
    { link: '/', label: 'Documents', icon: IconFileDescription },
    { link: '/content_types/', label: 'Content Types', icon: IconTopologyRing },
    { link: '/admin/', label: 'API & Admin', icon: IconApi },
    { link: '/settings/', label: 'Settings', icon: IconSettings },
];

export interface SidebarProps {
    height: string
}

export default function Sidebar({height}: SidebarProps) {
    const { classes, cx } = useStyles();
    const [active, setActive] = useState('Documents');

    const links = tabs.map((tab) => (
        <Link
            to={tab.link}
            className={cx(classes.link, { [classes.linkActive]: tab.label === active })}
            key={tab.label}
            onClick={() => {
                setActive(tab.label);
            }}
        >
            <tab.icon className={classes.linkIcon} stroke={1.5} />
            <span>{tab.label}</span>
        </Link>
    ));

    return (
        <Navbar height={height} width={{ sm: 300 }} p="md" style={{
            borderRight: `${rem(3)} solid #F3BAC0`
        }}>
            <Navbar.Section grow>
                {links}
            </Navbar.Section>

            <Navbar.Section className={classes.footer}>
                <a href="#" className={classes.link} onClick={(event) => event.preventDefault()}>
                    <IconSwitchHorizontal className={classes.linkIcon} stroke={1.5} />
                    <span>Change account</span>
                </a>

                <a href="#" className={classes.link} onClick={(event) => event.preventDefault()}>
                    <IconLogout className={classes.linkIcon} stroke={1.5} />
                    <span>Logout</span>
                </a>
            </Navbar.Section>
        </Navbar>
    );
}
