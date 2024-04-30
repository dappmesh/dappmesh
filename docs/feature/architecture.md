# Architecture

```mermaid
---
title: Data Mesh 
---
flowchart TB

    dataMesh[Data Mesh] --- domainA[Domain A]
    dataMesh[Data Mesh] --- domainB[Domain B]
    domainA --- productA
    domainB --- productB

    subgraph productA [Data Product A]
        direction TB

        subgraph productAGovernance [Governance]
            direction TB

            subgraph productASecurity [Security]
                direction LR

                productAIAM[IAM] -.- productAVault[Secret Manager]
            end

            subgraph productACatalog [Catalog]
                direction TB

                productASchema[Schema] -.- productAContract[Contract]
                productASchema[Schema] -.- productAVersioning[Versioning]
                productASchema[Schema] -.- productASnapshot[Snapshot]
            end

            productASecurity --- productACatalog
        end 

        subgraph productAProcessing [Processing]
            direction TB
            
            subgraph productAAPI [API]
                direction LR

                productAConsumer[Consumer] -.- productAProducer[Producer]
            end

            subgraph productAOrchestration [Orchestration]
                direction LR

                productAScheduler[Scheduler] -.- productAWorkflow[Workflow]
            end

            subgraph productAPipeline [Pipeline]
                direction LR

                productABatch[Batch] -.- productAStream[Stream]
            end
            
            productAAPI --- productAOrchestration
            productAOrchestration --- productAPipeline
        end

        subgraph productAStorage [Storage]
            direction LR

            productAOperationalDatabase[(Operational)]
            productAAnalyticsDatabase[(Analytics)]
            productADataLake[(Data Lake)]

            productAOperationalDatabase -.- productAAnalyticsDatabase
            productAAnalyticsDatabase -.- productADataLake
        end

        productAGovernance --- productAProcessing
        productAProcessing --- productAStorage
    end

    subgraph productB [Data Product B]
        direction TB

        subgraph productBGovernance [Governance]
            direction TB

            subgraph productBSecurity [Security]
                direction LR

                productBIAM[IAM] -.- productBVault[Secret Manager]
            end

            subgraph productBCatalog [Catalog]
                direction TB

                productBSchema[Schema] -.- productBContract[Contract]
                productBSchema[Schema] -.- productBVersioning[Versioning]
                productBSchema[Schema] -.- productBSnapshot[Snapshot]
            end

            productBSecurity --- productBCatalog
        end 

        subgraph productBProcessing [Processing]
            direction TB
            
            subgraph productBAPI [API]
                direction LR

                productBConsumer[Consumer] -.- productBProducer[Producer]
            end

            subgraph productBOrchestration [Orchestration]
                direction LR

                productBScheduler[Scheduler] -.- productBWorkflow[Workflow]
            end

            subgraph productBPipeline [Pipeline]
                direction LR

                productBBatch[Batch] -.- productBStream[Stream]
            end
            
            productBAPI --- productBOrchestration
            productBOrchestration --- productBPipeline
        end

        subgraph productBStorage [Storage]
            direction LR

            productBOperationalDatabase[(Operational)]
            productBAnalyticsDatabase[(Analytics)]
            productBDataLake[(Data Lake)]

            productBOperationalDatabase -.- productBAnalyticsDatabase
            productBAnalyticsDatabase -.- productBDataLake
        end

        productBGovernance --- productBProcessing
        productBProcessing --- productBStorage
    end 
```

## Entity Component System (ECS)


> *An ECS comprises entities composed from components of data, with systems which operate on entities' components.*
*ECS follows the principle of composition over inheritance, meaning that every entity is defined not by a type hierarchy, but by the components that are associated with it. Systems act globally over all entities which have the required components.*
[...]
*Common ECS approaches are highly compatible with, and are often combined with, data-oriented design techniques. Data for all instances of a component are commonly stored together in physical memory, enabling efficient memory access for systems which operate over many entities.*
[^1]

### Simple instruction multiple data (SIMD)
In a system that favors composition over inheritance we can maybe[^2] apply SIMD, which is a type of parallel processing used for data intensive efficient computations.[^3] 

[^1]: source: [Wikipedia](https://en.wikipedia.org/wiki/Entity_component_system)
[^2]: this is my intuition based on hobby level graphics and sound programming knowledge. These are two fields where SIMD is widely used to tune performance at the machine level.
[^3]: see the [*Energy Efficient Programming*](https://open.hpi.de/courses/cleanIT-x862022) online course.
