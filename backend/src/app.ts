import express from "express";
import cors from "cors";
import helmet from "helmet";
import { createApiRouter } from "./routes";
import { createAdminRouter } from "./routes/admin";
import { errorHandler } from "./middleware/errorHandler";
import { ApiKeyAuthProvider, requireAdmin, requireAuth } from "./middleware/auth";
import { PayoutsController } from "./controllers/payouts.controller";
import { WorkerController } from "./controllers/worker.controller";
import { AdminController } from "./controllers/admin.controller";
import { AuthController } from "./controllers/auth.controller";
import type { PaymentService } from "./services/paymentService";
import type { PaymentWorker } from "./workers/paymentWorker";
import type { TransactionRepository } from "./repositories/transactionRepository";
import type { AdminService } from "./services/adminService";
import type { AuthService } from "./services/authService";

export interface AppDependencies {
  paymentService: PaymentService;
  paymentWorker: PaymentWorker;
  transactions: TransactionRepository;
  adminService: AdminService;
  authService: AuthService;
}

export function createApp(deps: AppDependencies): express.Application {
  const app = express();

  // Configure Helmet with security headers
  app.use(
    helmet({
      // HSTS: Force HTTPS for 1 year, including subdomains
      hsts: {
        maxAge: 31536000, // 1 year in seconds
        includeSubDomains: true,
        preload: true,
      },
      // Referrer Policy: Balance privacy and analytics
      referrerPolicy: {
        policy: "strict-origin-when-cross-origin",
      },
      // Cross-Origin Opener Policy: Improve isolation
      crossOriginOpenerPolicy: {
        policy: "same-origin",
      },
      // Cross-Origin Resource Policy: Allow frontend to load from API
      crossOriginResourcePolicy: {
        policy: "cross-origin",
      },
      // Content Security Policy: Disabled (handled by Next.js frontend)
      contentSecurityPolicy: false,
      // Permitted Cross-Domain Policies: Disable Adobe Flash/PDF policies
      permittedCrossDomainPolicies: {
        permittedPolicies: "none",
      },
    })
  );
  app.use(cors());
  app.use(express.json());

  app.get("/health", (_req, res) => {
    res.json({ status: "ok" });
  });

  const payoutsController = new PayoutsController(deps.paymentService, deps.transactions);
  const workerController = new WorkerController(deps.paymentWorker);
  const adminController = new AdminController(
    deps.adminService,
    deps.paymentService,
    deps.transactions
  );
  const authController = new AuthController(deps.authService);

  const adminAuthMiddleware = requireAdmin(new ApiKeyAuthProvider());
  const userAuthMiddleware = requireAuth(deps.authService);

  app.use("/api", createApiRouter(payoutsController, workerController, authController, userAuthMiddleware));
  app.use("/api/admin", createAdminRouter(adminController, adminAuthMiddleware));

  app.use(errorHandler);

  return app;
}
