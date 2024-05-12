# Composite Operator Mesh

In its simplest form, DappMesh can be set up on a single Kubernetes cluster. For more complex use cases, it can be deployed in a multi-cluster architecture.

The goal of using the [Kubernetes Operator Framework](https://operatorframework.io/) to provision DappMesh custom resources is to automate management activities (installation, configuration, update, backup, failover, restore, etc) by integrating natively with Kubernetes APIs.

For this purpose, we are using [kube.rs](https://kube.rs/), a [Rust](https://www.rust-lang.org/) client and controller-runtime for Kubernetes, hosted by [CNCF](https://www.cncf.io/).

- **Mesh Operator:** It is responsible for managing the metadata database and provisioning the core infrastructure, such as the service mesh, certificate manager, key vault, identity management, and monitoring system. Additionally, it can synchronize inter-mesh metadata in multi-cluster scenarios.


- **Domain Operator:** An admission controller enforces the governance, security, policies, and validation of domain custom resources within the mesh. Similarly, the domain operator manages governance over data products within the domain.


- **Data Product Operator:** This operator manages the [application model's](./application-model.md) custom resources, such as schemas, contracts, and flows. Additionally, it provisions and controls the product infrastructure, including storage (operational, analytics, object storage) and processing (pipelines, orchestration, service APIs).

```mermaid
---
title: Data Mesh 
---
flowchart TB
    
    productCluster --- marketingCluster

    subgraph productCluster[Product Cluster]
    direction BT
        productMesh --- salesDomain
        salesDomain --- orderProduct
        salesDomain --- salesProduct
    
    
        subgraph productMesh[Product]
            direction TB
            productMeshOperator[Mesh Operator]
        end
        
        subgraph salesDomain[Sales]
            direction TB
            salesOperator[Domain Operator]
        end
    
        subgraph orderProduct[Customer Purchase History]
            direction TB
            crmProductOperator[Data Product Operator] 
        end
    
        subgraph salesProduct[Sales Forecasting]
            direction TB
            orderProductOperator[Data Product Operator]
        end
        
        productMesh --- supplyDomain
        supplyDomain --- inventoryProduct
        supplyDomain --- supplierProduct

        subgraph supplyDomain[Supply Chain]
            direction TB
            supplyOperator[Domain Operator]
        end

        subgraph inventoryProduct[Inventory Management]
            direction TB
            inventoryProductOperator[Data Product Operator]
        end

        subgraph supplierProduct[Supplier Performance]
            direction TB
            supplierProductOperator[Data Product Operator]
        end
    end

    subgraph marketingCluster[Marketing Cluster]
        direction TB
        marketingMesh --- campaignDomain
        campaignDomain --- campaignProduct
        campaignDomain --- trendsProduct

        subgraph marketingMesh[Marketing]
            direction TB
            marketingMeshOperator[Mesh Operator]
        end

        subgraph campaignDomain[Campaign]
            direction TB
            campaignOperator[Domain Operator]
        end

        subgraph campaignProduct[Campaign Analytics]
            direction TB
            campaignProductOperator[Data Product Operator]
        end

        subgraph trendsProduct[Trends & Sentiment Analysis]
            direction TB
            trendsProductOperator[Data Product Operator]
        end

        marketingMesh --- productInsightsDomain
        productInsightsDomain --- usageProduct
        productInsightsDomain --- marketFitProduct

        subgraph productInsightsDomain[Product Insights]
            direction TB
            productInsightsOperator[Domain Operator]
        end

        subgraph usageProduct[Product Usage Analytics]
            direction TB
            usageProductOperator[Data Product Operator]
        end

        subgraph marketFitProduct[Market Fit & Feedback]
            direction TB
            marketFitProductOperator[Data Product Operator]
        end
    end
```