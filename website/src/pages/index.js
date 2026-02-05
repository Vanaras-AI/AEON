import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Heading from '@theme/Heading';

import styles from './index.module.css';

function HomepageHeader() {
    const { siteConfig } = useDocusaurusContext();
    return (
        <header className={clsx('hero hero--primary', styles.heroBanner)}>
            <div className="container">
                <Heading as="h1" className="hero__title">
                    {siteConfig.title}
                </Heading>
                <p className="hero__subtitle">{siteConfig.tagline}</p>
                <div className={styles.buttons}>
                    <Link
                        className="button button--secondary button--lg"
                        to="/docs/intro">
                        Get Started →
                    </Link>
                </div>
            </div>
        </header>
    );
}

function Features() {
    return (
        <section className={styles.features}>
            <div className="container">
                <div className="row">
                    <div className="col col--4">
                        <div className="text--center padding-horiz--md">
                            <h3>🛡️ Governance-First</h3>
                            <p>
                                Every AI agent action passes through a 6-phase security pipeline
                                before execution.
                            </p>
                        </div>
                    </div>
                    <div className="col col--4">
                        <div className="text--center padding-horiz--md">
                            <h3>🧠 Hybrid Risk Scoring</h3>
                            <p>
                                Gemma 3 AI + battle-tested heuristics for 100% threat detection
                                with zero cloud dependency.
                            </p>
                        </div>
                    </div>
                    <div className="col col--4">
                        <div className="text--center padding-horiz--md">
                            <h3>🔒 WASM Isolation</h3>
                            <p>
                                Perfect security boundaries with sandboxed execution,
                                resource limits, and audit trails.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    );
}

export default function Home() {
    const { siteConfig } = useDocusaurusContext();
    return (
        <Layout
            title={`${siteConfig.title} - ${siteConfig.tagline}`}
            description="AEON - Governance-First AI Runtime. Secure, policy-enforced execution for AI agents.">
            <HomepageHeader />
            <main>
                <Features />
            </main>
        </Layout>
    );
}
